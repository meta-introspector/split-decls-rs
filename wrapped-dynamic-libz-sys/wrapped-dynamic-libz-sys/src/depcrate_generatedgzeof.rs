// Generated macro for gzeof (function)
macro_rules! Depcrate_generatedgzeof {
() => {
// Module: crate::generated
// Provides: {"gzeof"}
// Dependencies: {}
# [cfg (any (zng , feature = "libc"))] pub unsafe fn gzeof (file : gzFile) -> c_int { type Func = unsafe extern "C" fn (file : gzFile) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "gzeof") } , "gzeof") . as_bytes ()) . unwrap () ; f (file) }
};
}
