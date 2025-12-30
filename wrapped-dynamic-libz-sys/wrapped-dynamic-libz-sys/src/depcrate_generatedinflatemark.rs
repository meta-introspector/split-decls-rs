// Generated macro for inflateMark (function)
macro_rules! Depcrate_generatedinflateMark {
() => {
// Module: crate::generated
// Provides: {"inflateMark"}
// Dependencies: {}
pub unsafe fn inflateMark (strm : z_streamp) -> c_long { type Func = unsafe extern "C" fn (strm : z_streamp) -> c_long ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "inflateMark") } , "inflateMark") . as_bytes ()) . unwrap () ; f (strm) }
};
}
