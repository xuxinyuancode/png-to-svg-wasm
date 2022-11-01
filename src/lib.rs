mod utils;

use image::{
    codecs::jpeg::JpegEncoder, imageops::FilterType, load_from_memory, ColorType, GenericImageView,
};
use std::str;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn get_svg(raw: Vec<u8>) -> String {
    let img = load_from_memory(&raw).unwrap();
    let mut result = svg_header(img.width(), img.height());

    let mut temX: u32 = 0;
    let mut temY: u32 = 0;
    let mut temW: u32 = 0;
    let mut temH: u32 = 0;
    let mut temR: f32 = 0.0;
    let mut temG: f32 = 0.0;
    let mut temB: f32 = 0.0;
    let mut temA: f32 = 0.0;

    for x in 1..img.width() {
        for y in 1..img.height() {
            let rgba = img.get_pixel(x, y);

            let _r = rgba.0[0] as f32;
            let _g = rgba.0[1] as f32;
            let _b = rgba.0[2] as f32;
            let _a = rgba.0[3] as f32;

            temX = x;
            temY = y;
            temW = 1;
            temH = 1;
            temR = _r;
            temG = _g;
            temB = _b;
            temA = _a;
            result += &getRect(temX, temY, temW, temH, temR, temG, temB, temA);
        }
    }
    result += "</svg>";
    result
}

pub fn svg_header(width: u32, height: u32) -> String {
    let mut result = "".to_string();
    result += &"<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>
    <!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" 
      \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">
    <svg width=\"";
    result += (format!("{}", width)).as_str();
    result += "\" height=\"";
    result += (format!("{}", height)).as_str();
    result += "\" xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\">";
    return result;
}

pub fn getRect(x: u32, y: u32, w: u32, h: u32, _r: f32, _g: f32, _b: f32, _a: f32) -> String {
    let result = format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"1\" style=\"fill:rgb({}, {}, {}); fill-opacity:{};\" />",x,y,w,_r,_g,_b,_a);
    return result.to_string();
}
