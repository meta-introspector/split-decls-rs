// Generated macro for gzputs (function)
macro_rules! Depcrate_generatedgzputs {
() => {
// Module: crate::generated
// Provides: {"gzputs"}
// Dependencies: {}
# [cfg (any (zng , feature = "libc"))] pub unsafe fn gzputs (file : gzFile , s : * const c_char) -> c_int { type Func = unsafe extern "C" fn (file : gzFile , s : * const c_char) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "gzputs") } , "gzputs") . as_bytes ()) . unwrap () ; f (file , s) }
};
}
