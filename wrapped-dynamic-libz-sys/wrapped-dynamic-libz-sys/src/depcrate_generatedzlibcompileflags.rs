// Generated macro for zlibCompileFlags (function)
macro_rules! Depcrate_generatedzlibCompileFlags {
() => {
// Module: crate::generated
// Provides: {"zlibCompileFlags"}
// Dependencies: {}
pub unsafe fn zlibCompileFlags () -> uLong { type Func = unsafe extern "C" fn () -> uLong ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "zlibCompileFlags") } , "zlibCompileFlags") . as_bytes ()) . unwrap () ; f () }
};
}
