// Generated macro for deflate (function)
macro_rules! Depcrate_generateddeflate {
() => {
// Module: crate::generated
// Provides: {"deflate"}
// Dependencies: {}
pub unsafe fn deflate (strm : z_streamp , flush : c_int) -> c_int { type Func = unsafe extern "C" fn (strm : z_streamp , flush : c_int) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "deflate") } , "deflate") . as_bytes ()) . unwrap () ; f (strm , flush) }
};
}
