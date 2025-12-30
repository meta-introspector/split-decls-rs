// Generated macro for CRC_8_NRSC_5 (const)
macro_rules! Depcrate_algorithmCRC_8_NRSC_5 {
() => {
// Module: crate::algorithm
// Provides: {"CRC_8_NRSC_5"}
// Dependencies: {}
# [doc = " # [`CRC-8/NRSC-5`][1]"] # [doc = ""] # [doc = " - `width`: `8` bits"] # [doc = " - `poly`: `0x31` (reversed: `0x8c`)"] # [doc = " - `init`: `0xff`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0xf7`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-8-nrsc-5"] pub const CRC_8_NRSC_5 : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x31 , init : 0xff , refin : false , refout : false , xorout : 0x0 , check : 0xf7 , residue : 0x0 } ;
};
}
