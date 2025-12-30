// Generated macro for CRC_31_PHILIPS (const)
macro_rules! Depcrate_algorithmCRC_31_PHILIPS {
() => {
// Module: crate::algorithm
// Provides: {"CRC_31_PHILIPS"}
// Dependencies: {}
# [doc = " # [`CRC-31/PHILIPS`][1]"] # [doc = ""] # [doc = " - `width`: `31` bits"] # [doc = " - `poly`: `0x4c11db7` (reversed: `0x76dc4190`)"] # [doc = " - `init`: `0x7fffffff`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x7fffffff`"] # [doc = " - `check`: `0xce9e46c`"] # [doc = " - `residue`: `0x4eaf26f1`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-31-philips"] pub const CRC_31_PHILIPS : Algorithm < u32 > = Algorithm { width : 31 , poly : 0x4c11db7 , init : 0x7fffffff , refin : false , refout : false , xorout : 0x7fffffff , check : 0xce9e46c , residue : 0x4eaf26f1 } ;
};
}
