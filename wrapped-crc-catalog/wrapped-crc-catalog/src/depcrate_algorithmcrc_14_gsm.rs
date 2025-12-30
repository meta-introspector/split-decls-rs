// Generated macro for CRC_14_GSM (const)
macro_rules! Depcrate_algorithmCRC_14_GSM {
() => {
// Module: crate::algorithm
// Provides: {"CRC_14_GSM"}
// Dependencies: {}
# [doc = " # [`CRC-14/GSM`][1]"] # [doc = ""] # [doc = " - `width`: `14` bits"] # [doc = " - `poly`: `0x202d` (reversed: `0x2d01`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x3fff`"] # [doc = " - `check`: `0x30ae`"] # [doc = " - `residue`: `0x31e`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-14-gsm"] pub const CRC_14_GSM : Algorithm < u16 > = Algorithm { width : 14 , poly : 0x202d , init : 0x0 , refin : false , refout : false , xorout : 0x3fff , check : 0x30ae , residue : 0x31e } ;
};
}
