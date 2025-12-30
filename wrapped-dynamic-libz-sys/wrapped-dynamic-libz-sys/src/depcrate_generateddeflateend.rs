// Generated macro for deflateEnd (function)
macro_rules! Depcrate_generateddeflateEnd {
() => {
// Module: crate::generated
// Provides: {"deflateEnd"}
// Dependencies: {}
pub unsafe fn deflateEnd (strm : z_streamp) -> c_int { type Func = unsafe extern "C" fn (strm : z_streamp) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "deflateEnd") } , "deflateEnd") . as_bytes ()) . unwrap () ; f (strm) }
};
}
