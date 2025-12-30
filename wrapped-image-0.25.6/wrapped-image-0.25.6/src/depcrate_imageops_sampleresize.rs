// Generated macro for resize (function)
macro_rules! Depcrate_imageops_sampleresize {
() => {
// Module: crate::imageops::sample
// Provides: {"resize"}
// Dependencies: {}
# [doc = " Resize the supplied image to the specified dimensions."] # [doc = " ```nwidth``` and ```nheight``` are the new dimensions."] # [doc = " ```filter``` is the sampling filter to use."] # [doc = " This method assumes alpha pre-multiplication for images that contain non-constant alpha."] pub fn resize < I : GenericImageView > (image : & I , nwidth : u32 , nheight : u32 , filter : FilterType ,) -> ImageBuffer < I :: Pixel , Vec < < I :: Pixel as Pixel > :: Subpixel > > where I :: Pixel : 'static , < I :: Pixel as Pixel > :: Subpixel : 'static , { let is_empty = { let (width , height) = image . dimensions () ; width == 0 || height == 0 } ; if is_empty { return ImageBuffer :: new (nwidth , nheight) ; } if (nwidth , nheight) == image . dimensions () { let mut tmp = ImageBuffer :: new (image . width () , image . height ()) ; tmp . copy_from (image , 0 , 0) . unwrap () ; return tmp ; } let mut method = match filter { FilterType :: Nearest => Filter { kernel : Box :: new (box_kernel) , support : 0.0 , } , FilterType :: Triangle => Filter { kernel : Box :: new (triangle_kernel) , support : 1.0 , } , FilterType :: CatmullRom => Filter { kernel : Box :: new (catmullrom_kernel) , support : 2.0 , } , FilterType :: Gaussian => Filter { kernel : Box :: new (gaussian_kernel) , support : 3.0 , } , FilterType :: Lanczos3 => Filter { kernel : Box :: new (lanczos3_kernel) , support : 3.0 , } , } ; let tmp : Rgba32FImage = vertical_sample (image , nheight , & mut method) ; horizontal_sample (& tmp , nwidth , & mut method) }
};
}
