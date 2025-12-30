// Generated macro for deflateSetDictionary (function)
macro_rules! Depcrate_generateddeflateSetDictionary {
() => {
// Module: crate::generated
// Provides: {"deflateSetDictionary"}
// Dependencies: {}
pub unsafe fn deflateSetDictionary (strm : z_streamp , dictionary : * const Bytef , dictLength : uInt ,) -> c_int { type Func = unsafe extern "C" fn (strm : z_streamp , dictionary : * const Bytef , dictLength : uInt) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "deflateSetDictionary") } , "deflateSetDictionary") . as_bytes () ,) . unwrap () ; f (strm , dictionary , dictLength) }
};
}
