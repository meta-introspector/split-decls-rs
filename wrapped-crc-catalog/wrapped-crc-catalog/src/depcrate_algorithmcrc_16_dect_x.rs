// Generated macro for CRC_16_DECT_X (const)
macro_rules! Depcrate_algorithmCRC_16_DECT_X {
() => {
// Module: crate::algorithm
// Provides: {"CRC_16_DECT_X"}
// Dependencies: {}
# [doc = " # [`CRC-16/DECT-X`][1]"] # [doc = ""] # [doc = " - `width`: `16` bits"] # [doc = " - `poly`: `0x589` (reversed: `0x91a0`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x7f`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-16-dect-x"] pub const CRC_16_DECT_X : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x589 , init : 0x0 , refin : false , refout : false , xorout : 0x0 , check : 0x7f , residue : 0x0 } ;
};
}
