// Generated macro for gzclearerr (function)
macro_rules! Depcrate_generatedgzclearerr {
() => {
// Module: crate::generated
// Provides: {"gzclearerr"}
// Dependencies: {}
# [cfg (any (zng , feature = "libc"))] pub unsafe fn gzclearerr (file : gzFile) { type Func = unsafe extern "C" fn (file : gzFile) ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "gzclearerr") } , "gzclearerr") . as_bytes ()) . unwrap () ; f (file) }
};
}
