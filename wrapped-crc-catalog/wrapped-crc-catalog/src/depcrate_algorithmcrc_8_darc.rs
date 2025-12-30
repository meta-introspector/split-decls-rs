// Generated macro for CRC_8_DARC (const)
macro_rules! Depcrate_algorithmCRC_8_DARC {
() => {
// Module: crate::algorithm
// Provides: {"CRC_8_DARC"}
// Dependencies: {}
# [doc = " # [`CRC-8/DARC`][1]"] # [doc = ""] # [doc = " - `width`: `8` bits"] # [doc = " - `poly`: `0x39` (reversed: `0x9c`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x15`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-8-darc"] pub const CRC_8_DARC : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x39 , init : 0x0 , refin : true , refout : true , xorout : 0x0 , check : 0x15 , residue : 0x0 } ;
};
}
