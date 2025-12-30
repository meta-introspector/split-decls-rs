// Generated macro for CRC_8_CDMA2000 (const)
macro_rules! Depcrate_algorithmCRC_8_CDMA2000 {
() => {
// Module: crate::algorithm
// Provides: {"CRC_8_CDMA2000"}
// Dependencies: {}
# [doc = " # [`CRC-8/CDMA2000`][1]"] # [doc = ""] # [doc = " - `width`: `8` bits"] # [doc = " - `poly`: `0x9b` (reversed: `0xd9`)"] # [doc = " - `init`: `0xff`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0xda`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-8-cdma2000"] pub const CRC_8_CDMA2000 : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x9b , init : 0xff , refin : false , refout : false , xorout : 0x0 , check : 0xda , residue : 0x0 } ;
};
}
