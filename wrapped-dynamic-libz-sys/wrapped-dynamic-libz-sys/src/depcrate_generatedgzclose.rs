// Generated macro for gzclose (function)
macro_rules! Depcrate_generatedgzclose {
() => {
// Module: crate::generated
// Provides: {"gzclose"}
// Dependencies: {}
# [cfg (any (zng , feature = "libc"))] pub unsafe fn gzclose (file : gzFile) -> c_int { type Func = unsafe extern "C" fn (file : gzFile) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "gzclose") } , "gzclose") . as_bytes ()) . unwrap () ; f (file) }
};
}
