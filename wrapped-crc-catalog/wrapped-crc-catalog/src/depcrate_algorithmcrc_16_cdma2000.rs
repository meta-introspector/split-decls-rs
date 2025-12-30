// Generated macro for CRC_16_CDMA2000 (const)
macro_rules! Depcrate_algorithmCRC_16_CDMA2000 {
() => {
// Module: crate::algorithm
// Provides: {"CRC_16_CDMA2000"}
// Dependencies: {}
# [doc = " # [`CRC-16/CDMA2000`][1]"] # [doc = ""] # [doc = " - `width`: `16` bits"] # [doc = " - `poly`: `0xc867` (reversed: `0xe613`)"] # [doc = " - `init`: `0xffff`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x4c06`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-16-cdma2000"] pub const CRC_16_CDMA2000 : Algorithm < u16 > = Algorithm { width : 16 , poly : 0xc867 , init : 0xffff , refin : false , refout : false , xorout : 0x0 , check : 0x4c06 , residue : 0x0 } ;
};
}
