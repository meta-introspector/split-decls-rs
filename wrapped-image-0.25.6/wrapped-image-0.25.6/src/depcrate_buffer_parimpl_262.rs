// Generated macro for impl_262 (impl)
macro_rules! Depcrate_buffer_parimpl_262 {
() => {
// Module: crate::buffer_par
// Provides: {"impl_262"}
// Dependencies: {}
impl < P , Container > ImageBuffer < P , Container > where P : Pixel + Send + Sync , P :: Subpixel : Send + Sync , Container : Deref < Target = [P :: Subpixel] > + DerefMut , { # [doc = " Returns a parallel iterator over the mutable pixels of this image, usable with `rayon`."] # [doc = " See [`pixels_mut`] for more information."] # [doc = ""] # [doc = " [`pixels_mut`]: #method.pixels_mut"] pub fn par_pixels_mut (& mut self) -> PixelsMutPar < P > { PixelsMutPar { chunks : self . inner_pixels_mut () . par_chunks_exact_mut (< P as Pixel > :: CHANNEL_COUNT as usize) , } } # [doc = " Returns a parallel iterator over the mutable pixels of this image and their coordinates, usable with `rayon`."] # [doc = " See [`enumerate_pixels_mut`] for more information."] # [doc = ""] # [doc = " [`enumerate_pixels_mut`]: #method.enumerate_pixels_mut"] pub fn par_enumerate_pixels_mut (& mut self) -> EnumeratePixelsMutPar < P > { let width = self . width () ; EnumeratePixelsMutPar { pixels : self . par_pixels_mut () , width , } } }
};
}
