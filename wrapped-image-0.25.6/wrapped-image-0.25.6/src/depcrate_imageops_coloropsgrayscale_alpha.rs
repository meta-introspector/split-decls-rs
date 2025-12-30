// Generated macro for grayscale_alpha (function)
macro_rules! Depcrate_imageops_coloropsgrayscale_alpha {
() => {
// Module: crate::imageops::colorops
// Provides: {"grayscale_alpha"}
// Dependencies: {}
# [doc = " Convert the supplied image to grayscale. Alpha channel is preserved."] pub fn grayscale_alpha < I : GenericImageView > (image : & I ,) -> ImageBuffer < LumaA < Subpixel < I > > , Vec < Subpixel < I > > > { grayscale_with_type_alpha (image) }
};
}
