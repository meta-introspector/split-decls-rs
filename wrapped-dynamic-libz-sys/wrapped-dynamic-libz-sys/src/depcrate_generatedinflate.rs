// Generated macro for inflate (function)
macro_rules! Depcrate_generatedinflate {
() => {
// Module: crate::generated
// Provides: {"inflate"}
// Dependencies: {}
pub unsafe fn inflate (strm : z_streamp , flush : c_int) -> c_int { type Func = unsafe extern "C" fn (strm : z_streamp , flush : c_int) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "inflate") } , "inflate") . as_bytes ()) . unwrap () ; f (strm , flush) }
};
}
