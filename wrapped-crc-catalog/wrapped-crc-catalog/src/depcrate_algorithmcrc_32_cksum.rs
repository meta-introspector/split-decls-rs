// Generated macro for CRC_32_CKSUM (const)
macro_rules! Depcrate_algorithmCRC_32_CKSUM {
() => {
// Module: crate::algorithm
// Provides: {"CRC_32_CKSUM"}
// Dependencies: {}
# [doc = " # [`CRC-32/CKSUM`][1]"] # [doc = ""] # [doc = " - `width`: `32` bits"] # [doc = " - `poly`: `0x4c11db7` (reversed: `0xedb88320`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0xffffffff`"] # [doc = " - `check`: `0x765e7680`"] # [doc = " - `residue`: `0xc704dd7b`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-32-cksum"] pub const CRC_32_CKSUM : Algorithm < u32 > = Algorithm { width : 32 , poly : 0x4c11db7 , init : 0x0 , refin : false , refout : false , xorout : 0xffffffff , check : 0x765e7680 , residue : 0xc704dd7b } ;
};
}
