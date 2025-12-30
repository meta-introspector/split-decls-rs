// Generated macro for deflateCopy (function)
macro_rules! Depcrate_generateddeflateCopy {
() => {
// Module: crate::generated
// Provides: {"deflateCopy"}
// Dependencies: {}
pub unsafe fn deflateCopy (dest : z_streamp , source : z_streamp) -> c_int { type Func = unsafe extern "C" fn (dest : z_streamp , source : z_streamp) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "deflateCopy") } , "deflateCopy") . as_bytes ()) . unwrap () ; f (dest , source) }
};
}
