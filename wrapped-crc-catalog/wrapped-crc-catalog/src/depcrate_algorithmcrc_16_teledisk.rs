// Generated macro for CRC_16_TELEDISK (const)
macro_rules! Depcrate_algorithmCRC_16_TELEDISK {
() => {
// Module: crate::algorithm
// Provides: {"CRC_16_TELEDISK"}
// Dependencies: {}
# [doc = " # [`CRC-16/TELEDISK`][1]"] # [doc = ""] # [doc = " - `width`: `16` bits"] # [doc = " - `poly`: `0xa097` (reversed: `0xe905`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0xfb3`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-16-teledisk"] pub const CRC_16_TELEDISK : Algorithm < u16 > = Algorithm { width : 16 , poly : 0xa097 , init : 0x0 , refin : false , refout : false , xorout : 0x0 , check : 0xfb3 , residue : 0x0 } ;
};
}
