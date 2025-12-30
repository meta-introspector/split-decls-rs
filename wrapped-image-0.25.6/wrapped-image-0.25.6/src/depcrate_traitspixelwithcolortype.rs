// Generated macro for PixelWithColorType (trait)
macro_rules! Depcrate_traitsPixelWithColorType {
() => {
// Module: crate::traits
// Provides: {"PixelWithColorType"}
// Dependencies: {}
# [doc = " The pixel with an associated `ColorType`."] # [doc = " Not all possible pixels represent one of the predefined `ColorType`s."] pub trait PixelWithColorType : Pixel + private :: SealedPixelWithColorType { # [doc = " This pixel has the format of one of the predefined `ColorType`s,"] # [doc = " such as `Rgb8`, `La16` or `Rgba32F`."] # [doc = " This is needed for automatically detecting"] # [doc = " a color format when saving an image as a file."] const COLOR_TYPE : ExtendedColorType ; }
};
}
