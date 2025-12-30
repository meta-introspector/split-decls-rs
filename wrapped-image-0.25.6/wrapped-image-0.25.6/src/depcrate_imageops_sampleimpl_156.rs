// Generated macro for impl_156 (impl)
macro_rules! Depcrate_imageops_sampleimpl_156 {
() => {
// Module: crate::imageops::sample
// Provides: {"impl_156"}
// Dependencies: {}
impl < S : Primitive + Enlargeable > ThumbnailSum < S > { fn zeroed () -> Self { ThumbnailSum (S :: Larger :: zero () , S :: Larger :: zero () , S :: Larger :: zero () , S :: Larger :: zero () ,) } fn sample_val (val : S) -> S :: Larger { < S :: Larger as NumCast > :: from (val) . unwrap () } fn add_pixel < P : Pixel < Subpixel = S > > (& mut self , pixel : P) { # [allow (deprecated)] let pixel = pixel . channels4 () ; self . 0 += Self :: sample_val (pixel . 0) ; self . 1 += Self :: sample_val (pixel . 1) ; self . 2 += Self :: sample_val (pixel . 2) ; self . 3 += Self :: sample_val (pixel . 3) ; } }
};
}
