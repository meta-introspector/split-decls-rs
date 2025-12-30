// Generated macro for gzputc (function)
macro_rules! Depcrate_generatedgzputc {
() => {
// Module: crate::generated
// Provides: {"gzputc"}
// Dependencies: {}
# [cfg (any (zng , feature = "libc"))] pub unsafe fn gzputc (file : gzFile , c : c_int) -> c_int { type Func = unsafe extern "C" fn (file : gzFile , c : c_int) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "gzputc") } , "gzputc") . as_bytes ()) . unwrap () ; f (file , c) }
};
}
