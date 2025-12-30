// Generated macro for deflateBound (function)
macro_rules! Depcrate_generateddeflateBound {
() => {
// Module: crate::generated
// Provides: {"deflateBound"}
// Dependencies: {}
pub unsafe fn deflateBound (strm : z_streamp , sourceLen : uLong) -> uLong { type Func = unsafe extern "C" fn (strm : z_streamp , sourceLen : uLong) -> uLong ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "deflateBound") } , "deflateBound") . as_bytes ()) . unwrap () ; f (strm , sourceLen) }
};
}
