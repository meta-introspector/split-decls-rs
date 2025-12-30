// Generated macro for deflatePrime (function)
macro_rules! Depcrate_generateddeflatePrime {
() => {
// Module: crate::generated
// Provides: {"deflatePrime"}
// Dependencies: {}
pub unsafe fn deflatePrime (strm : z_streamp , bits : c_int , value : c_int) -> c_int { type Func = unsafe extern "C" fn (strm : z_streamp , bits : c_int , value : c_int) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "deflatePrime") } , "deflatePrime") . as_bytes ()) . unwrap () ; f (strm , bits , value) }
};
}
