// Generated macro for impl_271 (impl)
macro_rules! Depcrate_colorimpl_271 {
() => {
// Module: crate::color
// Provides: {"impl_271"}
// Dependencies: {}
impl ColorType { # [doc = " Returns the number of bytes contained in a pixel of `ColorType` ```c```"] # [must_use] pub fn bytes_per_pixel (self) -> u8 { match self { ColorType :: L8 => 1 , ColorType :: L16 | ColorType :: La8 => 2 , ColorType :: Rgb8 => 3 , ColorType :: Rgba8 | ColorType :: La16 => 4 , ColorType :: Rgb16 => 6 , ColorType :: Rgba16 => 8 , ColorType :: Rgb32F => 3 * 4 , ColorType :: Rgba32F => 4 * 4 , } } # [doc = " Returns if there is an alpha channel."] # [must_use] pub fn has_alpha (self) -> bool { use ColorType :: * ; match self { L8 | L16 | Rgb8 | Rgb16 | Rgb32F => false , La8 | Rgba8 | La16 | Rgba16 | Rgba32F => true , } } # [doc = " Returns false if the color scheme is grayscale, true otherwise."] # [must_use] pub fn has_color (self) -> bool { use ColorType :: * ; match self { L8 | L16 | La8 | La16 => false , Rgb8 | Rgb16 | Rgba8 | Rgba16 | Rgb32F | Rgba32F => true , } } # [doc = " Returns the number of bits contained in a pixel of `ColorType` ```c``` (which will always be"] # [doc = " a multiple of 8)."] # [must_use] pub fn bits_per_pixel (self) -> u16 { < u16 as From < u8 > > :: from (self . bytes_per_pixel ()) * 8 } # [doc = " Returns the number of color channels that make up this pixel"] # [must_use] pub fn channel_count (self) -> u8 { let e : ExtendedColorType = self . into () ; e . channel_count () } }
};
}
