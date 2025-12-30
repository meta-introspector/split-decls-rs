// Generated macro for CRC_8_LTE (const)
macro_rules! Depcrate_algorithmCRC_8_LTE {
() => {
// Module: crate::algorithm
// Provides: {"CRC_8_LTE"}
// Dependencies: {}
# [doc = " # [`CRC-8/LTE`][1]"] # [doc = ""] # [doc = " - `width`: `8` bits"] # [doc = " - `poly`: `0x9b` (reversed: `0xd9`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0xea`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-8-lte"] pub const CRC_8_LTE : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x9b , init : 0x0 , refin : false , refout : false , xorout : 0x0 , check : 0xea , residue : 0x0 } ;
};
}
