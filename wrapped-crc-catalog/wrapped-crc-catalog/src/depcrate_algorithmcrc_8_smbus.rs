// Generated macro for CRC_8_SMBUS (const)
macro_rules! Depcrate_algorithmCRC_8_SMBUS {
() => {
// Module: crate::algorithm
// Provides: {"CRC_8_SMBUS"}
// Dependencies: {}
# [doc = " # [`CRC-8/SMBUS`][1]"] # [doc = ""] # [doc = " - `width`: `8` bits"] # [doc = " - `poly`: `0x7` (reversed: `0xe0`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0xf4`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-8-smbus"] pub const CRC_8_SMBUS : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x7 , init : 0x0 , refin : false , refout : false , xorout : 0x0 , check : 0xf4 , residue : 0x0 } ;
};
}
