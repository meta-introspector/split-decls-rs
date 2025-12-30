// Generated macro for gzsetparams (function)
macro_rules! Depcrate_generatedgzsetparams {
() => {
// Module: crate::generated
// Provides: {"gzsetparams"}
// Dependencies: {}
# [cfg (any (zng , feature = "libc"))] pub unsafe fn gzsetparams (file : gzFile , level : c_int , strategy : c_int) -> c_int { type Func = unsafe extern "C" fn (file : gzFile , level : c_int , strategy : c_int) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "gzsetparams") } , "gzsetparams") . as_bytes ()) . unwrap () ; f (file , level , strategy) }
};
}
