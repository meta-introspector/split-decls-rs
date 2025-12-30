// Generated macro for inflateCopy (function)
macro_rules! Depcrate_generatedinflateCopy {
() => {
// Module: crate::generated
// Provides: {"inflateCopy"}
// Dependencies: {}
pub unsafe fn inflateCopy (dest : z_streamp , source : z_streamp) -> c_int { type Func = unsafe extern "C" fn (dest : z_streamp , source : z_streamp) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "inflateCopy") } , "inflateCopy") . as_bytes ()) . unwrap () ; f (dest , source) }
};
}
