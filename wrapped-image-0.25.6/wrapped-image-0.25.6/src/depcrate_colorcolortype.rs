// Generated macro for ColorType (enum)
macro_rules! Depcrate_colorColorType {
() => {
// Module: crate::color
// Provides: {"ColorType"}
// Dependencies: {}
# [doc = " An enumeration over supported color types and bit depths"] # [derive (Copy , PartialEq , Eq , Debug , Clone , Hash)] # [non_exhaustive] pub enum ColorType { # [doc = " Pixel is 8-bit luminance"] L8 , # [doc = " Pixel is 8-bit luminance with an alpha channel"] La8 , # [doc = " Pixel contains 8-bit R, G and B channels"] Rgb8 , # [doc = " Pixel is 8-bit RGB with an alpha channel"] Rgba8 , # [doc = " Pixel is 16-bit luminance"] L16 , # [doc = " Pixel is 16-bit luminance with an alpha channel"] La16 , # [doc = " Pixel is 16-bit RGB"] Rgb16 , # [doc = " Pixel is 16-bit RGBA"] Rgba16 , # [doc = " Pixel is 32-bit float RGB"] Rgb32F , # [doc = " Pixel is 32-bit float RGBA"] Rgba32F , }
};
}
