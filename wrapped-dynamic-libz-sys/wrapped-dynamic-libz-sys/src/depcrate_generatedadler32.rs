// Generated macro for adler32 (function)
macro_rules! Depcrate_generatedadler32 {
() => {
// Module: crate::generated
// Provides: {"adler32"}
// Dependencies: {}
pub unsafe fn adler32 (adler : z_checksum , buf : * const Bytef , len : uInt) -> z_checksum { type Func = unsafe extern "C" fn (adler : z_checksum , buf : * const Bytef , len : uInt) -> z_checksum ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "adler32") } , "adler32") . as_bytes ()) . unwrap () ; f (adler , buf , len) }
};
}
