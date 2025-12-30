// Generated macro for deflateReset (function)
macro_rules! Depcrate_generateddeflateReset {
() => {
// Module: crate::generated
// Provides: {"deflateReset"}
// Dependencies: {}
pub unsafe fn deflateReset (strm : z_streamp) -> c_int { type Func = unsafe extern "C" fn (strm : z_streamp) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "deflateReset") } , "deflateReset") . as_bytes ()) . unwrap () ; f (strm) }
};
}
