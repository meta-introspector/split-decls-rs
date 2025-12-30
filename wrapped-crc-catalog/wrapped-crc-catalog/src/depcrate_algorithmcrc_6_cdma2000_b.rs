// Generated macro for CRC_6_CDMA2000_B (const)
macro_rules! Depcrate_algorithmCRC_6_CDMA2000_B {
() => {
// Module: crate::algorithm
// Provides: {"CRC_6_CDMA2000_B"}
// Dependencies: {}
# [doc = " # [`CRC-6/CDMA2000-B`][1]"] # [doc = ""] # [doc = " - `width`: `6` bits"] # [doc = " - `poly`: `0x7` (reversed: `0x38`)"] # [doc = " - `init`: `0x3f`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x3b`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-6-cdma2000-b"] pub const CRC_6_CDMA2000_B : Algorithm < u8 > = Algorithm { width : 6 , poly : 0x7 , init : 0x3f , refin : false , refout : false , xorout : 0x0 , check : 0x3b , residue : 0x0 } ;
};
}
