// Generated macro for dither (function)
macro_rules! Depcrate_imageops_coloropsdither {
() => {
// Module: crate::imageops::colorops
// Provides: {"dither"}
// Dependencies: {}
# [doc = " Reduces the colors of the image using the supplied `color_map` while applying"] # [doc = " Floyd-Steinberg dithering to improve the visual conception"] pub fn dither < Pix , Map > (image : & mut ImageBuffer < Pix , Vec < u8 > > , color_map : & Map) where Map : ColorMap < Color = Pix > + ? Sized , Pix : Pixel < Subpixel = u8 > + 'static , { let (width , height) = image . dimensions () ; let mut err : [i16 ; 3] = [0 ; 3] ; for y in 0 .. height - 1 { let x = 0 ; do_dithering ! (color_map , image , err , x , y) ; diffuse_err (image . get_pixel_mut (x + 1 , y) , err , 7) ; diffuse_err (image . get_pixel_mut (x , y + 1) , err , 5) ; diffuse_err (image . get_pixel_mut (x + 1 , y + 1) , err , 1) ; for x in 1 .. width - 1 { do_dithering ! (color_map , image , err , x , y) ; diffuse_err (image . get_pixel_mut (x + 1 , y) , err , 7) ; diffuse_err (image . get_pixel_mut (x - 1 , y + 1) , err , 3) ; diffuse_err (image . get_pixel_mut (x , y + 1) , err , 5) ; diffuse_err (image . get_pixel_mut (x + 1 , y + 1) , err , 1) ; } let x = width - 1 ; do_dithering ! (color_map , image , err , x , y) ; diffuse_err (image . get_pixel_mut (x - 1 , y + 1) , err , 3) ; diffuse_err (image . get_pixel_mut (x , y + 1) , err , 5) ; } let y = height - 1 ; let x = 0 ; do_dithering ! (color_map , image , err , x , y) ; diffuse_err (image . get_pixel_mut (x + 1 , y) , err , 7) ; for x in 1 .. width - 1 { do_dithering ! (color_map , image , err , x , y) ; diffuse_err (image . get_pixel_mut (x + 1 , y) , err , 7) ; } let x = width - 1 ; do_dithering ! (color_map , image , err , x , y) ; }
};
}
