// Generated macro for BiLevel (struct)
macro_rules! Depcrate_imageops_coloropsBiLevel {
() => {
// Module: crate::imageops::colorops
// Provides: {"BiLevel"}
// Dependencies: {}
# [doc = " A bi-level color map"] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " use image::imageops::colorops::{index_colors, BiLevel, ColorMap};"] # [doc = " use image::{ImageBuffer, Luma};"] # [doc = ""] # [doc = " let (w, h) = (16, 16);"] # [doc = " // Create an image with a smooth horizontal gradient from black (0) to white (255)."] # [doc = " let gray = ImageBuffer::from_fn(w, h, |x, y| -> Luma<u8> { [(255 * x / w) as u8].into() });"] # [doc = " // Mapping the gray image through the `BiLevel` filter should map gray pixels less than half"] # [doc = " // intensity (127) to black (0), and anything greater to white (255)."] # [doc = " let cmap = BiLevel;"] # [doc = " let palletized = index_colors(&gray, &cmap);"] # [doc = " let mapped = ImageBuffer::from_fn(w, h, |x, y| {"] # [doc = "     let p = palletized.get_pixel(x, y);"] # [doc = "     cmap.lookup(p.0[0] as usize)"] # [doc = "         .expect(\"indexed color out-of-range\")"] # [doc = " });"] # [doc = " // Create an black and white image of expected output."] # [doc = " let bw = ImageBuffer::from_fn(w, h, |x, y| -> Luma<u8> {"] # [doc = "     if x <= (w / 2) {"] # [doc = "         [0].into()"] # [doc = "     } else {"] # [doc = "         [255].into()"] # [doc = "     }"] # [doc = " });"] # [doc = " assert_eq!(mapped, bw);"] # [doc = " ```"] # [derive (Clone , Copy)] pub struct BiLevel ;
};
}
