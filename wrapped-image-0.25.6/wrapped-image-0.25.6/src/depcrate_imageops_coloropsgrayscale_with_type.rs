// Generated macro for grayscale_with_type (function)
macro_rules! Depcrate_imageops_coloropsgrayscale_with_type {
() => {
// Module: crate::imageops::colorops
// Provides: {"grayscale_with_type"}
// Dependencies: {}
# [doc = " Convert the supplied image to a grayscale image with the specified pixel type. Alpha channel is discarded."] pub fn grayscale_with_type < NewPixel , I : GenericImageView > (image : & I ,) -> ImageBuffer < NewPixel , Vec < NewPixel :: Subpixel > > where NewPixel : Pixel + FromColor < Luma < Subpixel < I > > > , { let (width , height) = image . dimensions () ; let mut out = ImageBuffer :: new (width , height) ; for (x , y , pixel) in image . pixels () { let grayscale = pixel . to_luma () ; let new_pixel = grayscale . into_color () ; out . put_pixel (x , y , new_pixel) ; } out }
};
}
