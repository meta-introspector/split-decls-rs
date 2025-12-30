// Generated macro for CRC_4_INTERLAKEN (const)
macro_rules! Depcrate_algorithmCRC_4_INTERLAKEN {
() => {
// Module: crate::algorithm
// Provides: {"CRC_4_INTERLAKEN"}
// Dependencies: {}
# [doc = " # [`CRC-4/INTERLAKEN`][1]"] # [doc = ""] # [doc = " - `width`: `4` bits"] # [doc = " - `poly`: `0x3` (reversed: `0xc`)"] # [doc = " - `init`: `0xf`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0xf`"] # [doc = " - `check`: `0xb`"] # [doc = " - `residue`: `0x2`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-4-interlaken"] pub const CRC_4_INTERLAKEN : Algorithm < u8 > = Algorithm { width : 4 , poly : 0x3 , init : 0xf , refin : false , refout : false , xorout : 0xf , check : 0xb , residue : 0x2 } ;
};
}
