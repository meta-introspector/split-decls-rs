// Generated macro for compress (function)
macro_rules! Depcrate_generatedcompress {
() => {
// Module: crate::generated
// Provides: {"compress"}
// Dependencies: {}
# [cfg (any (zng , feature = "libc"))] pub unsafe fn compress (dest : * mut Bytef , destLen : * mut z_size , source : * const Bytef , sourceLen : z_size ,) -> c_int { type Func = unsafe extern "C" fn (dest : * mut Bytef , destLen : * mut z_size , source : * const Bytef , sourceLen : z_size ,) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "compress") } , "compress") . as_bytes ()) . unwrap () ; f (dest , destLen , source , sourceLen) }
};
}
