// Generated macro for deflateSetHeader (function)
macro_rules! Depcrate_generateddeflateSetHeader {
() => {
// Module: crate::generated
// Provides: {"deflateSetHeader"}
// Dependencies: {}
pub unsafe fn deflateSetHeader (strm : z_streamp , head : gz_headerp) -> c_int { type Func = unsafe extern "C" fn (strm : z_streamp , head : gz_headerp) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "deflateSetHeader") } , "deflateSetHeader") . as_bytes ()) . unwrap () ; f (strm , head) }
};
}
