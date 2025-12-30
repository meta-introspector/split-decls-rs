// Generated macro for impl_255 (impl)
macro_rules! Depcrate_imageimpl_255 {
() => {
// Module: crate::image
// Provides: {"impl_255"}
// Dependencies: {}
impl CGImageRef { pub fn width (& self) -> usize { unsafe { CGImageGetWidth (self . as_ptr ()) } } pub fn height (& self) -> usize { unsafe { CGImageGetHeight (self . as_ptr ()) } } pub fn bits_per_component (& self) -> usize { unsafe { CGImageGetBitsPerComponent (self . as_ptr ()) } } pub fn bits_per_pixel (& self) -> usize { unsafe { CGImageGetBitsPerPixel (self . as_ptr ()) } } pub fn bytes_per_row (& self) -> usize { unsafe { CGImageGetBytesPerRow (self . as_ptr ()) } } pub fn color_space (& self) -> CGColorSpace { unsafe { let cs = CGImageGetColorSpace (self . as_ptr ()) ; CFRetain (cs as * mut _) ; CGColorSpace :: from_ptr (cs) } } # [doc = " Returns the raw image bytes wrapped in `CFData`. Note, the returned `CFData` owns the"] # [doc = " underlying buffer."] pub fn data (& self) -> CFData { let data_provider = unsafe { CGDataProviderRef :: from_ptr (CGImageGetDataProvider (self . as_ptr ())) } ; data_provider . copy_data () } # [doc = " Returns a cropped image. If the `rect` specifies a rectangle which lies outside of the"] # [doc = " image bounds, the `None` is returned."] pub fn cropped (& self , rect : CGRect) -> Option < CGImage > { let image_ptr = unsafe { CGImageCreateWithImageInRect (self . as_ptr () , rect) } ; if ! image_ptr . is_null () { Some (unsafe { CGImage :: from_ptr (image_ptr) }) } else { None } } }
};
}
