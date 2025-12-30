// Generated macro for crc32 (function)
macro_rules! Depcrate_generatedcrc32 {
() => {
// Module: crate::generated
// Provides: {"crc32"}
// Dependencies: {}
pub unsafe fn crc32 (crc : z_checksum , buf : * const Bytef , len : uInt) -> z_checksum { type Func = unsafe extern "C" fn (crc : z_checksum , buf : * const Bytef , len : uInt) -> z_checksum ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "crc32") } , "crc32") . as_bytes ()) . unwrap () ; f (crc , buf , len) }
};
}
