// Generated macro for CRC_8_OPENSAFETY (const)
macro_rules! Depcrate_algorithmCRC_8_OPENSAFETY {
() => {
// Module: crate::algorithm
// Provides: {"CRC_8_OPENSAFETY"}
// Dependencies: {}
# [doc = " # [`CRC-8/OPENSAFETY`][1]"] # [doc = ""] # [doc = " - `width`: `8` bits"] # [doc = " - `poly`: `0x2f` (reversed: `0xf4`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x3e`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-8-opensafety"] pub const CRC_8_OPENSAFETY : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x2f , init : 0x0 , refin : false , refout : false , xorout : 0x0 , check : 0x3e , residue : 0x0 } ;
};
}
