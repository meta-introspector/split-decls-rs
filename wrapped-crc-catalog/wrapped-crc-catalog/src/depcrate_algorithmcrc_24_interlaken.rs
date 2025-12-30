// Generated macro for CRC_24_INTERLAKEN (const)
macro_rules! Depcrate_algorithmCRC_24_INTERLAKEN {
() => {
// Module: crate::algorithm
// Provides: {"CRC_24_INTERLAKEN"}
// Dependencies: {}
# [doc = " # [`CRC-24/INTERLAKEN`][1]"] # [doc = ""] # [doc = " - `width`: `24` bits"] # [doc = " - `poly`: `0x328b63` (reversed: `0xc6d14c`)"] # [doc = " - `init`: `0xffffff`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0xffffff`"] # [doc = " - `check`: `0xb4f3e6`"] # [doc = " - `residue`: `0x144e63`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-24-interlaken"] pub const CRC_24_INTERLAKEN : Algorithm < u32 > = Algorithm { width : 24 , poly : 0x328b63 , init : 0xffffff , refin : false , refout : false , xorout : 0xffffff , check : 0xb4f3e6 , residue : 0x144e63 } ;
};
}
