// Generated macro for inflateSetDictionary (function)
macro_rules! Depcrate_generatedinflateSetDictionary {
() => {
// Module: crate::generated
// Provides: {"inflateSetDictionary"}
// Dependencies: {}
pub unsafe fn inflateSetDictionary (strm : z_streamp , dictionary : * const Bytef , dictLength : uInt ,) -> c_int { type Func = unsafe extern "C" fn (strm : z_streamp , dictionary : * const Bytef , dictLength : uInt) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "inflateSetDictionary") } , "inflateSetDictionary") . as_bytes () ,) . unwrap () ; f (strm , dictionary , dictLength) }
};
}
