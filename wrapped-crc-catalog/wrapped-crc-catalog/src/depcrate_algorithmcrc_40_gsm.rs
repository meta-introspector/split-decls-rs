// Generated macro for CRC_40_GSM (const)
macro_rules! Depcrate_algorithmCRC_40_GSM {
() => {
// Module: crate::algorithm
// Provides: {"CRC_40_GSM"}
// Dependencies: {}
# [doc = " # [`CRC-40/GSM`][1]"] # [doc = ""] # [doc = " - `width`: `40` bits"] # [doc = " - `poly`: `0x4820009` (reversed: `0x9000412000`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0xffffffffff`"] # [doc = " - `check`: `0xd4164fc646`"] # [doc = " - `residue`: `0xc4ff8071ff`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-40-gsm"] pub const CRC_40_GSM : Algorithm < u64 > = Algorithm { width : 40 , poly : 0x4820009 , init : 0x0 , refin : false , refout : false , xorout : 0xffffffffff , check : 0xd4164fc646 , residue : 0xc4ff8071ff } ;
};
}
