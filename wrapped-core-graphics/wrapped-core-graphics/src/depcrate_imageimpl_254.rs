// Generated macro for impl_254 (impl)
macro_rules! Depcrate_imageimpl_254 {
() => {
// Module: crate::image
// Provides: {"impl_254"}
// Dependencies: {}
impl CGImage { pub fn new (width : usize , height : usize , bits_per_component : usize , bits_per_pixel : usize , bytes_per_row : usize , colorspace : & CGColorSpace , bitmap_info : u32 , provider : & CGDataProvider , should_interpolate : bool , rendering_intent : u32 ,) -> Self { unsafe { let result = CGImageCreate (width , height , bits_per_component , bits_per_pixel , bytes_per_row , colorspace . as_ptr () , bitmap_info , provider . as_ptr () , ptr :: null_mut () , should_interpolate , rendering_intent ,) ; assert ! (! result . is_null ()) ; Self :: from_ptr (result) } } pub fn type_id () -> CFTypeID { unsafe { CGImageGetTypeID () } } }
};
}
