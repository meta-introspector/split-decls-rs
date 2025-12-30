// Generated macro for CRC_3_GSM (const)
macro_rules! Depcrate_algorithmCRC_3_GSM {
() => {
// Module: crate::algorithm
// Provides: {"CRC_3_GSM"}
// Dependencies: {}
# [doc = " # [`CRC-3/GSM`][1]"] # [doc = ""] # [doc = " - `width`: `3` bits"] # [doc = " - `poly`: `0x3` (reversed: `0x6`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x7`"] # [doc = " - `check`: `0x4`"] # [doc = " - `residue`: `0x2`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-3-gsm"] pub const CRC_3_GSM : Algorithm < u8 > = Algorithm { width : 3 , poly : 0x3 , init : 0x0 , refin : false , refout : false , xorout : 0x7 , check : 0x4 , residue : 0x2 } ;
};
}
