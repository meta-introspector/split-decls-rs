// Generated macro for CRC_8_WCDMA (const)
macro_rules! Depcrate_algorithmCRC_8_WCDMA {
() => {
// Module: crate::algorithm
// Provides: {"CRC_8_WCDMA"}
// Dependencies: {}
# [doc = " # [`CRC-8/WCDMA`][1]"] # [doc = ""] # [doc = " - `width`: `8` bits"] # [doc = " - `poly`: `0x9b` (reversed: `0xd9`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x25`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-8-wcdma"] pub const CRC_8_WCDMA : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x9b , init : 0x0 , refin : true , refout : true , xorout : 0x0 , check : 0x25 , residue : 0x0 } ;
};
}
