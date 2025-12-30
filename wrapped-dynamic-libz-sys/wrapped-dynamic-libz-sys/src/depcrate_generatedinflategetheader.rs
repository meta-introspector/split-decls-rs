// Generated macro for inflateGetHeader (function)
macro_rules! Depcrate_generatedinflateGetHeader {
() => {
// Module: crate::generated
// Provides: {"inflateGetHeader"}
// Dependencies: {}
pub unsafe fn inflateGetHeader (strm : z_streamp , head : gz_headerp) -> c_int { type Func = unsafe extern "C" fn (strm : z_streamp , head : gz_headerp) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "inflateGetHeader") } , "inflateGetHeader") . as_bytes ()) . unwrap () ; f (strm , head) }
};
}
