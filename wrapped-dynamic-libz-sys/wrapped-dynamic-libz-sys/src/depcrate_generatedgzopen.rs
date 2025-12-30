// Generated macro for gzopen (function)
macro_rules! Depcrate_generatedgzopen {
() => {
// Module: crate::generated
// Provides: {"gzopen"}
// Dependencies: {}
# [cfg (any (zng , feature = "libc"))] pub unsafe fn gzopen (path : * const c_char , mode : * const c_char) -> gzFile { type Func = unsafe extern "C" fn (path : * const c_char , mode : * const c_char) -> gzFile ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "gzopen") } , "gzopen") . as_bytes ()) . unwrap () ; f (path , mode) }
};
}
