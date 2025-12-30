// Generated macro for inflateBackEnd (function)
macro_rules! Depcrate_generatedinflateBackEnd {
() => {
// Module: crate::generated
// Provides: {"inflateBackEnd"}
// Dependencies: {}
pub unsafe fn inflateBackEnd (strm : z_streamp) -> c_int { type Func = unsafe extern "C" fn (strm : z_streamp) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "inflateBackEnd") } , "inflateBackEnd") . as_bytes ()) . unwrap () ; f (strm) }
};
}
