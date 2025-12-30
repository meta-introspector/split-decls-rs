// Generated macro for crc32_combine (function)
macro_rules! Depcrate_generatedcrc32_combine {
() => {
// Module: crate::generated
// Provides: {"crc32_combine"}
// Dependencies: {}
# [cfg (any (zng , feature = "libc"))] pub unsafe fn crc32_combine (crc1 : z_checksum , crc2 : z_checksum , len2 : z_off_t) -> z_checksum { type Func = unsafe extern "C" fn (crc1 : z_checksum , crc2 : z_checksum , len2 : z_off_t) -> z_checksum ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "crc32_combine") } , "crc32_combine") . as_bytes ()) . unwrap () ; f (crc1 , crc2 , len2) }
};
}
