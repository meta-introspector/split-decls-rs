// Generated macro for CRC_32_MPEG_2 (const)
macro_rules! Depcrate_algorithmCRC_32_MPEG_2 {
() => {
// Module: crate::algorithm
// Provides: {"CRC_32_MPEG_2"}
// Dependencies: {}
# [doc = " # [`CRC-32/MPEG-2`][1]"] # [doc = ""] # [doc = " - `width`: `32` bits"] # [doc = " - `poly`: `0x4c11db7` (reversed: `0xedb88320`)"] # [doc = " - `init`: `0xffffffff`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x376e6e7`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-32-mpeg-2"] pub const CRC_32_MPEG_2 : Algorithm < u32 > = Algorithm { width : 32 , poly : 0x4c11db7 , init : 0xffffffff , refin : false , refout : false , xorout : 0x0 , check : 0x376e6e7 , residue : 0x0 } ;
};
}
