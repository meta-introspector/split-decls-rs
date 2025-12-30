// Generated macro for compress2 (function)
macro_rules! Depcrate_generatedcompress2 {
() => {
// Module: crate::generated
// Provides: {"compress2"}
// Dependencies: {}
# [cfg (any (zng , feature = "libc"))] pub unsafe fn compress2 (dest : * mut Bytef , destLen : * mut z_size , source : * const Bytef , sourceLen : z_size , level : c_int ,) -> c_int { type Func = unsafe extern "C" fn (dest : * mut Bytef , destLen : * mut z_size , source : * const Bytef , sourceLen : z_size , level : c_int ,) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "compress2") } , "compress2") . as_bytes ()) . unwrap () ; f (dest , destLen , source , sourceLen , level) }
};
}
