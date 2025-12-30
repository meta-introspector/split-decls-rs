// Generated macro for compressBound (function)
macro_rules! Depcrate_generatedcompressBound {
() => {
// Module: crate::generated
// Provides: {"compressBound"}
// Dependencies: {}
# [cfg (any (zng , feature = "libc"))] pub unsafe fn compressBound (sourceLen : z_size) -> z_size { type Func = unsafe extern "C" fn (sourceLen : z_size) -> z_size ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "compressBound") } , "compressBound") . as_bytes ()) . unwrap () ; f (sourceLen) }
};
}
