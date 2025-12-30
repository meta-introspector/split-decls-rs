// Generated macro for CRC_32_JAMCRC (const)
macro_rules! Depcrate_algorithmCRC_32_JAMCRC {
() => {
// Module: crate::algorithm
// Provides: {"CRC_32_JAMCRC"}
// Dependencies: {}
# [doc = " # [`CRC-32/JAMCRC`][1]"] # [doc = ""] # [doc = " - `width`: `32` bits"] # [doc = " - `poly`: `0x4c11db7` (reversed: `0xedb88320`)"] # [doc = " - `init`: `0xffffffff`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x340bc6d9`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-32-jamcrc"] pub const CRC_32_JAMCRC : Algorithm < u32 > = Algorithm { width : 32 , poly : 0x4c11db7 , init : 0xffffffff , refin : true , refout : true , xorout : 0x0 , check : 0x340bc6d9 , residue : 0x0 } ;
};
}
