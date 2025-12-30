// Generated macro for gzseek (function)
macro_rules! Depcrate_generatedgzseek {
() => {
// Module: crate::generated
// Provides: {"gzseek"}
// Dependencies: {}
# [cfg (any (zng , feature = "libc"))] pub unsafe fn gzseek (file : gzFile , offset : z_off_t , whence : c_int) -> z_off_t { type Func = unsafe extern "C" fn (file : gzFile , offset : z_off_t , whence : c_int) -> z_off_t ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "gzseek") } , "gzseek") . as_bytes ()) . unwrap () ; f (file , offset , whence) }
};
}
