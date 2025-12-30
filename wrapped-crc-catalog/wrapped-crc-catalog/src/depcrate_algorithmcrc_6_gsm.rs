// Generated macro for CRC_6_GSM (const)
macro_rules! Depcrate_algorithmCRC_6_GSM {
() => {
// Module: crate::algorithm
// Provides: {"CRC_6_GSM"}
// Dependencies: {}
# [doc = " # [`CRC-6/GSM`][1]"] # [doc = ""] # [doc = " - `width`: `6` bits"] # [doc = " - `poly`: `0x2f` (reversed: `0x3d`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x3f`"] # [doc = " - `check`: `0x13`"] # [doc = " - `residue`: `0x3a`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-6-gsm"] pub const CRC_6_GSM : Algorithm < u8 > = Algorithm { width : 6 , poly : 0x2f , init : 0x0 , refin : false , refout : false , xorout : 0x3f , check : 0x13 , residue : 0x3a } ;
};
}
