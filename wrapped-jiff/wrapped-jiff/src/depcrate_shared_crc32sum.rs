// Generated macro for sum (function)
macro_rules! Depcrate_shared_crc32sum {
() => {
// Module: crate::shared::crc32
// Provides: {"sum"}
// Dependencies: {}
# [doc = " Returns the \"masked\" CRC32 checksum of the slice using the Castagnoli"] # [doc = " polynomial."] # [doc = ""] # [doc = " This \"masked\" checksum is the same one used by the Snappy frame format."] # [doc = " Masking is supposed to make the checksum robust with respect to data that"] # [doc = " contains the checksum itself."] pub (crate) fn sum (buf : & [u8]) -> u32 { let sum = slice16 (0 , buf) ; (sum . wrapping_shr (15) | sum . wrapping_shl (17)) . wrapping_add (0xA282EAD8) }
};
}
