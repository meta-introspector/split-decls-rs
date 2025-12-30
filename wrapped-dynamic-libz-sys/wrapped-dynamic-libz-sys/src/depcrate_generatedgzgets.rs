// Generated macro for gzgets (function)
macro_rules! Depcrate_generatedgzgets {
() => {
// Module: crate::generated
// Provides: {"gzgets"}
// Dependencies: {}
# [cfg (any (zng , feature = "libc"))] pub unsafe fn gzgets (file : gzFile , buf : * mut c_char , len : c_int) -> * mut c_char { type Func = unsafe extern "C" fn (file : gzFile , buf : * mut c_char , len : c_int) -> * mut c_char ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "gzgets") } , "gzgets") . as_bytes ()) . unwrap () ; f (file , buf , len) }
};
}
