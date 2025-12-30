// Generated macro for gzrewind (function)
macro_rules! Depcrate_generatedgzrewind {
() => {
// Module: crate::generated
// Provides: {"gzrewind"}
// Dependencies: {}
# [cfg (any (zng , feature = "libc"))] pub unsafe fn gzrewind (file : gzFile) -> c_int { type Func = unsafe extern "C" fn (file : gzFile) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "gzrewind") } , "gzrewind") . as_bytes ()) . unwrap () ; f (file) }
};
}
