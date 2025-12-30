// Generated macro for CRC_8_AUTOSAR (const)
macro_rules! Depcrate_algorithmCRC_8_AUTOSAR {
() => {
// Module: crate::algorithm
// Provides: {"CRC_8_AUTOSAR"}
// Dependencies: {}
# [doc = " # [`CRC-8/AUTOSAR`][1]"] # [doc = ""] # [doc = " - `width`: `8` bits"] # [doc = " - `poly`: `0x2f` (reversed: `0xf4`)"] # [doc = " - `init`: `0xff`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0xff`"] # [doc = " - `check`: `0xdf`"] # [doc = " - `residue`: `0x42`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-8-autosar"] pub const CRC_8_AUTOSAR : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x2f , init : 0xff , refin : false , refout : false , xorout : 0xff , check : 0xdf , residue : 0x42 } ;
};
}
