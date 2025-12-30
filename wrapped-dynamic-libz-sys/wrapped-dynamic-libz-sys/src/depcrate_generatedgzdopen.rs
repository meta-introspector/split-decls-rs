// Generated macro for gzdopen (function)
macro_rules! Depcrate_generatedgzdopen {
() => {
// Module: crate::generated
// Provides: {"gzdopen"}
// Dependencies: {}
# [cfg (any (zng , feature = "libc"))] pub unsafe fn gzdopen (fd : c_int , mode : * const c_char) -> gzFile { type Func = unsafe extern "C" fn (fd : c_int , mode : * const c_char) -> gzFile ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "gzdopen") } , "gzdopen") . as_bytes ()) . unwrap () ; f (fd , mode) }
};
}
