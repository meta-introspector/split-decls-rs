// Generated macro for gzflush (function)
macro_rules! Depcrate_generatedgzflush {
() => {
// Module: crate::generated
// Provides: {"gzflush"}
// Dependencies: {}
# [cfg (any (zng , feature = "libc"))] pub unsafe fn gzflush (file : gzFile , flush : c_int) -> c_int { type Func = unsafe extern "C" fn (file : gzFile , flush : c_int) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "gzflush") } , "gzflush") . as_bytes ()) . unwrap () ; f (file , flush) }
};
}
