// Generated macro for index_colors (function)
macro_rules! Depcrate_imageops_coloropsindex_colors {
() => {
// Module: crate::imageops::colorops
// Provides: {"index_colors"}
// Dependencies: {}
# [doc = " Reduces the colors using the supplied `color_map` and returns an image of the indices"] pub fn index_colors < Pix , Map > (image : & ImageBuffer < Pix , Vec < u8 > > , color_map : & Map ,) -> ImageBuffer < Luma < u8 > , Vec < u8 > > where Map : ColorMap < Color = Pix > + ? Sized , Pix : Pixel < Subpixel = u8 > + 'static , { let mut indices = ImageBuffer :: new (image . width () , image . height ()) ; for (pixel , idx) in image . pixels () . zip (indices . pixels_mut ()) { * idx = Luma ([color_map . index_of (pixel) as u8]) ; } indices }
};
}
