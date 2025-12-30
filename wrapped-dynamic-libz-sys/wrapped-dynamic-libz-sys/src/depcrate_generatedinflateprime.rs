// Generated macro for inflatePrime (function)
macro_rules! Depcrate_generatedinflatePrime {
() => {
// Module: crate::generated
// Provides: {"inflatePrime"}
// Dependencies: {}
pub unsafe fn inflatePrime (strm : z_streamp , bits : c_int , value : c_int) -> c_int { type Func = unsafe extern "C" fn (strm : z_streamp , bits : c_int , value : c_int) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "inflatePrime") } , "inflatePrime") . as_bytes ()) . unwrap () ; f (strm , bits , value) }
};
}
