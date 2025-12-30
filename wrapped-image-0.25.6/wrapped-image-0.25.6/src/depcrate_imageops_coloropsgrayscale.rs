// Generated macro for grayscale (function)
macro_rules! Depcrate_imageops_coloropsgrayscale {
() => {
// Module: crate::imageops::colorops
// Provides: {"grayscale"}
// Dependencies: {}
# [doc = " Convert the supplied image to grayscale. Alpha channel is discarded."] pub fn grayscale < I : GenericImageView > (image : & I ,) -> ImageBuffer < Luma < Subpixel < I > > , Vec < Subpixel < I > > > { grayscale_with_type (image) }
};
}
