// Generated macro for CRC_16_EN_13757 (const)
macro_rules! Depcrate_algorithmCRC_16_EN_13757 {
() => {
// Module: crate::algorithm
// Provides: {"CRC_16_EN_13757"}
// Dependencies: {}
# [doc = " # [`CRC-16/EN-13757`][1]"] # [doc = ""] # [doc = " - `width`: `16` bits"] # [doc = " - `poly`: `0x3d65` (reversed: `0xa6bc`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0xffff`"] # [doc = " - `check`: `0xc2b7`"] # [doc = " - `residue`: `0xa366`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-16-en-13757"] pub const CRC_16_EN_13757 : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x3d65 , init : 0x0 , refin : false , refout : false , xorout : 0xffff , check : 0xc2b7 , residue : 0xa366 } ;
};
}
