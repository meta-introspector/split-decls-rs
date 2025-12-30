// Generated macro for CRC_16_DECT_R (const)
macro_rules! Depcrate_algorithmCRC_16_DECT_R {
() => {
// Module: crate::algorithm
// Provides: {"CRC_16_DECT_R"}
// Dependencies: {}
# [doc = " # [`CRC-16/DECT-R`][1]"] # [doc = ""] # [doc = " - `width`: `16` bits"] # [doc = " - `poly`: `0x589` (reversed: `0x91a0`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x1`"] # [doc = " - `check`: `0x7e`"] # [doc = " - `residue`: `0x589`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-16-dect-r"] pub const CRC_16_DECT_R : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x589 , init : 0x0 , refin : false , refout : false , xorout : 0x1 , check : 0x7e , residue : 0x589 } ;
};
}
