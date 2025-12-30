// Generated macro for gztell (function)
macro_rules! Depcrate_generatedgztell {
() => {
// Module: crate::generated
// Provides: {"gztell"}
// Dependencies: {}
# [cfg (any (zng , feature = "libc"))] pub unsafe fn gztell (file : gzFile) -> z_off_t { type Func = unsafe extern "C" fn (file : gzFile) -> z_off_t ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "gztell") } , "gztell") . as_bytes ()) . unwrap () ; f (file) }
};
}
