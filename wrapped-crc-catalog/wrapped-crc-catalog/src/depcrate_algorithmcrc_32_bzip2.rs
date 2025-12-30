// Generated macro for CRC_32_BZIP2 (const)
macro_rules! Depcrate_algorithmCRC_32_BZIP2 {
() => {
// Module: crate::algorithm
// Provides: {"CRC_32_BZIP2"}
// Dependencies: {}
# [doc = " # [`CRC-32/BZIP2`][1]"] # [doc = ""] # [doc = " - `width`: `32` bits"] # [doc = " - `poly`: `0x4c11db7` (reversed: `0xedb88320`)"] # [doc = " - `init`: `0xffffffff`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0xffffffff`"] # [doc = " - `check`: `0xfc891918`"] # [doc = " - `residue`: `0xc704dd7b`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-32-bzip2"] pub const CRC_32_BZIP2 : Algorithm < u32 > = Algorithm { width : 32 , poly : 0x4c11db7 , init : 0xffffffff , refin : false , refout : false , xorout : 0xffffffff , check : 0xfc891918 , residue : 0xc704dd7b } ;
};
}
