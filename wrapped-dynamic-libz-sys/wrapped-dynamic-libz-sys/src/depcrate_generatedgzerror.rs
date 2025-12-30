// Generated macro for gzerror (function)
macro_rules! Depcrate_generatedgzerror {
() => {
// Module: crate::generated
// Provides: {"gzerror"}
// Dependencies: {}
# [cfg (any (zng , feature = "libc"))] pub unsafe fn gzerror (file : gzFile , errnum : * mut c_int) -> * const c_char { type Func = unsafe extern "C" fn (file : gzFile , errnum : * mut c_int) -> * const c_char ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "gzerror") } , "gzerror") . as_bytes ()) . unwrap () ; f (file , errnum) }
};
}
