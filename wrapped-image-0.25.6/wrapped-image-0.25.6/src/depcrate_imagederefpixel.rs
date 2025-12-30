// Generated macro for DerefPixel (type)
macro_rules! Depcrate_imageDerefPixel {
() => {
// Module: crate::image
// Provides: {"DerefPixel"}
// Dependencies: {}
# [doc = " Alias to access Pixel behind a reference"] type DerefPixel < I > = < < I as Deref > :: Target as GenericImageView > :: Pixel ;
};
}
