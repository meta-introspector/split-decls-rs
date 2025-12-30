// Generated macro for CRC_15_MPT1327 (const)
macro_rules! Depcrate_algorithmCRC_15_MPT1327 {
() => {
// Module: crate::algorithm
// Provides: {"CRC_15_MPT1327"}
// Dependencies: {}
# [doc = " # [`CRC-15/MPT1327`][1]"] # [doc = ""] # [doc = " - `width`: `15` bits"] # [doc = " - `poly`: `0x6815` (reversed: `0x540b`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x1`"] # [doc = " - `check`: `0x2566`"] # [doc = " - `residue`: `0x6815`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-15-mpt1327"] pub const CRC_15_MPT1327 : Algorithm < u16 > = Algorithm { width : 15 , poly : 0x6815 , init : 0x0 , refin : false , refout : false , xorout : 0x1 , check : 0x2566 , residue : 0x6815 } ;
};
}
