// Generated macro for gzdirect (function)
macro_rules! Depcrate_generatedgzdirect {
() => {
// Module: crate::generated
// Provides: {"gzdirect"}
// Dependencies: {}
# [cfg (any (zng , feature = "libc"))] pub unsafe fn gzdirect (file : gzFile) -> c_int { type Func = unsafe extern "C" fn (file : gzFile) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "gzdirect") } , "gzdirect") . as_bytes ()) . unwrap () ; f (file) }
};
}
