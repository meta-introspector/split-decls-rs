// Generated macro for CRC_16_T10_DIF (const)
macro_rules! Depcrate_algorithmCRC_16_T10_DIF {
() => {
// Module: crate::algorithm
// Provides: {"CRC_16_T10_DIF"}
// Dependencies: {}
# [doc = " # [`CRC-16/T10-DIF`][1]"] # [doc = ""] # [doc = " - `width`: `16` bits"] # [doc = " - `poly`: `0x8bb7` (reversed: `0xedd1`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0xd0db`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-16-t10-dif"] pub const CRC_16_T10_DIF : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x8bb7 , init : 0x0 , refin : false , refout : false , xorout : 0x0 , check : 0xd0db , residue : 0x0 } ;
};
}
