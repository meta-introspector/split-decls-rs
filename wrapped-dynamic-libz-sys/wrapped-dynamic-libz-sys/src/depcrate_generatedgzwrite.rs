// Generated macro for gzwrite (function)
macro_rules! Depcrate_generatedgzwrite {
() => {
// Module: crate::generated
// Provides: {"gzwrite"}
// Dependencies: {}
# [cfg (any (zng , feature = "libc"))] pub unsafe fn gzwrite (file : gzFile , buf : voidpc , len : c_uint) -> c_int { type Func = unsafe extern "C" fn (file : gzFile , buf : voidpc , len : c_uint) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "gzwrite") } , "gzwrite") . as_bytes ()) . unwrap () ; f (file , buf , len) }
};
}
