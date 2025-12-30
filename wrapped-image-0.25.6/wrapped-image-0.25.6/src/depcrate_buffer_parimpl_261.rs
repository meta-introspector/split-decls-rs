// Generated macro for impl_261 (impl)
macro_rules! Depcrate_buffer_parimpl_261 {
() => {
// Module: crate::buffer_par
// Provides: {"impl_261"}
// Dependencies: {}
impl < P , Container > ImageBuffer < P , Container > where P : Pixel + Sync , P :: Subpixel : Sync , Container : Deref < Target = [P :: Subpixel] > , { # [doc = " Returns a parallel iterator over the pixels of this image, usable with `rayon`."] # [doc = " See [`pixels`] for more information."] # [doc = ""] # [doc = " [`pixels`]: #method.pixels"] pub fn par_pixels (& self) -> PixelsPar < P > { PixelsPar { chunks : self . inner_pixels () . par_chunks_exact (< P as Pixel > :: CHANNEL_COUNT as usize) , } } # [doc = " Returns a parallel iterator over the pixels of this image and their coordinates, usable with `rayon`."] # [doc = " See [`enumerate_pixels`] for more information."] # [doc = ""] # [doc = " [`enumerate_pixels`]: #method.enumerate_pixels"] pub fn par_enumerate_pixels (& self) -> EnumeratePixelsPar < P > { EnumeratePixelsPar { pixels : self . par_pixels () , width : self . width () , } } }
};
}
