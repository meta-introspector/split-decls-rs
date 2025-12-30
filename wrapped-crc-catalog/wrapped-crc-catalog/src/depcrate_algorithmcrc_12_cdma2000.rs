// Generated macro for CRC_12_CDMA2000 (const)
macro_rules! Depcrate_algorithmCRC_12_CDMA2000 {
() => {
// Module: crate::algorithm
// Provides: {"CRC_12_CDMA2000"}
// Dependencies: {}
# [doc = " # [`CRC-12/CDMA2000`][1]"] # [doc = ""] # [doc = " - `width`: `12` bits"] # [doc = " - `poly`: `0xf13` (reversed: `0xc8f`)"] # [doc = " - `init`: `0xfff`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0xd4d`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-12-cdma2000"] pub const CRC_12_CDMA2000 : Algorithm < u16 > = Algorithm { width : 12 , poly : 0xf13 , init : 0xfff , refin : false , refout : false , xorout : 0x0 , check : 0xd4d , residue : 0x0 } ;
};
}
