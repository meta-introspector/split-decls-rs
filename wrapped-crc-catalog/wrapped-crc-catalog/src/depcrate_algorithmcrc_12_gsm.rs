// Generated macro for CRC_12_GSM (const)
macro_rules! Depcrate_algorithmCRC_12_GSM {
() => {
// Module: crate::algorithm
// Provides: {"CRC_12_GSM"}
// Dependencies: {}
# [doc = " # [`CRC-12/GSM`][1]"] # [doc = ""] # [doc = " - `width`: `12` bits"] # [doc = " - `poly`: `0xd31` (reversed: `0x8cb`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0xfff`"] # [doc = " - `check`: `0xb34`"] # [doc = " - `residue`: `0x178`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-12-gsm"] pub const CRC_12_GSM : Algorithm < u16 > = Algorithm { width : 12 , poly : 0xd31 , init : 0x0 , refin : false , refout : false , xorout : 0xfff , check : 0xb34 , residue : 0x178 } ;
};
}
