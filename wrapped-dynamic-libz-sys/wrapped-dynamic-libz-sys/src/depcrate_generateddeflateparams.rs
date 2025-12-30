// Generated macro for deflateParams (function)
macro_rules! Depcrate_generateddeflateParams {
() => {
// Module: crate::generated
// Provides: {"deflateParams"}
// Dependencies: {}
pub unsafe fn deflateParams (strm : z_streamp , level : c_int , strategy : c_int) -> c_int { type Func = unsafe extern "C" fn (strm : z_streamp , level : c_int , strategy : c_int) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "deflateParams") } , "deflateParams") . as_bytes ()) . unwrap () ; f (strm , level , strategy) }
};
}
