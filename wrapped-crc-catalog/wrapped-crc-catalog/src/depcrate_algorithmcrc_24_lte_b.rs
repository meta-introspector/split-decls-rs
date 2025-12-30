// Generated macro for CRC_24_LTE_B (const)
macro_rules! Depcrate_algorithmCRC_24_LTE_B {
() => {
// Module: crate::algorithm
// Provides: {"CRC_24_LTE_B"}
// Dependencies: {}
# [doc = " # [`CRC-24/LTE-B`][1]"] # [doc = ""] # [doc = " - `width`: `24` bits"] # [doc = " - `poly`: `0x800063` (reversed: `0xc60001`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x23ef52`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-24-lte-b"] pub const CRC_24_LTE_B : Algorithm < u32 > = Algorithm { width : 24 , poly : 0x800063 , init : 0x0 , refin : false , refout : false , xorout : 0x0 , check : 0x23ef52 , residue : 0x0 } ;
};
}
