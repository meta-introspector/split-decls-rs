// Generated macro for inflateReset2 (function)
macro_rules! Depcrate_generatedinflateReset2 {
() => {
// Module: crate::generated
// Provides: {"inflateReset2"}
// Dependencies: {}
pub unsafe fn inflateReset2 (strm : z_streamp , windowBits : c_int) -> c_int { type Func = unsafe extern "C" fn (strm : z_streamp , windowBits : c_int) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "inflateReset2") } , "inflateReset2") . as_bytes ()) . unwrap () ; f (strm , windowBits) }
};
}
