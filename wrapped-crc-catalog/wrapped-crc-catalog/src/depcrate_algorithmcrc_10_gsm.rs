// Generated macro for CRC_10_GSM (const)
macro_rules! Depcrate_algorithmCRC_10_GSM {
() => {
// Module: crate::algorithm
// Provides: {"CRC_10_GSM"}
// Dependencies: {}
# [doc = " # [`CRC-10/GSM`][1]"] # [doc = ""] # [doc = " - `width`: `10` bits"] # [doc = " - `poly`: `0x175` (reversed: `0x2ba`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x3ff`"] # [doc = " - `check`: `0x12a`"] # [doc = " - `residue`: `0xc6`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-10-gsm"] pub const CRC_10_GSM : Algorithm < u16 > = Algorithm { width : 10 , poly : 0x175 , init : 0x0 , refin : false , refout : false , xorout : 0x3ff , check : 0x12a , residue : 0xc6 } ;
};
}
