// Generated macro for gzgetc (function)
macro_rules! Depcrate_generatedgzgetc {
() => {
// Module: crate::generated
// Provides: {"gzgetc"}
// Dependencies: {}
# [cfg (any (zng , feature = "libc"))] pub unsafe fn gzgetc (file : gzFile) -> c_int { type Func = unsafe extern "C" fn (file : gzFile) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "gzgetc") } , "gzgetc") . as_bytes ()) . unwrap () ; f (file) }
};
}
