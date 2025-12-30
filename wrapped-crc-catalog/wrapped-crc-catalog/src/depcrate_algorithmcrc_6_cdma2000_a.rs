// Generated macro for CRC_6_CDMA2000_A (const)
macro_rules! Depcrate_algorithmCRC_6_CDMA2000_A {
() => {
// Module: crate::algorithm
// Provides: {"CRC_6_CDMA2000_A"}
// Dependencies: {}
# [doc = " # [`CRC-6/CDMA2000-A`][1]"] # [doc = ""] # [doc = " - `width`: `6` bits"] # [doc = " - `poly`: `0x27` (reversed: `0x39`)"] # [doc = " - `init`: `0x3f`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0xd`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-6-cdma2000-a"] pub const CRC_6_CDMA2000_A : Algorithm < u8 > = Algorithm { width : 6 , poly : 0x27 , init : 0x3f , refin : false , refout : false , xorout : 0x0 , check : 0xd , residue : 0x0 } ;
};
}
