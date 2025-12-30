// Generated macro for gzungetc (function)
macro_rules! Depcrate_generatedgzungetc {
() => {
// Module: crate::generated
// Provides: {"gzungetc"}
// Dependencies: {}
# [cfg (any (zng , feature = "libc"))] pub unsafe fn gzungetc (c : c_int , file : gzFile) -> c_int { type Func = unsafe extern "C" fn (c : c_int , file : gzFile) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "gzungetc") } , "gzungetc") . as_bytes ()) . unwrap () ; f (c , file) }
};
}
