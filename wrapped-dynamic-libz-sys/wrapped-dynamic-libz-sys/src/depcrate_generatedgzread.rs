// Generated macro for gzread (function)
macro_rules! Depcrate_generatedgzread {
() => {
// Module: crate::generated
// Provides: {"gzread"}
// Dependencies: {}
# [cfg (any (zng , feature = "libc"))] pub unsafe fn gzread (file : gzFile , buf : voidp , len : c_uint) -> c_int { type Func = unsafe extern "C" fn (file : gzFile , buf : voidp , len : c_uint) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "gzread") } , "gzread") . as_bytes ()) . unwrap () ; f (file , buf , len) }
};
}
