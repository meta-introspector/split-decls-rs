// Generated macro for zlibVersion (function)
macro_rules! Depcrate_generatedzlibVersion {
() => {
// Module: crate::generated
// Provides: {"zlibVersion"}
// Dependencies: {}
pub unsafe fn zlibVersion () -> * const c_char { type Func = unsafe extern "C" fn () -> * const c_char ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "zlibVersion") } , "zlibVersion") . as_bytes ()) . unwrap () ; f () }
};
}
