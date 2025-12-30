// Generated macro for CRC_16_ARC (const)
macro_rules! Depcrate_algorithmCRC_16_ARC {
() => {
// Module: crate::algorithm
// Provides: {"CRC_16_ARC"}
// Dependencies: {}
# [doc = " # [`CRC-16/ARC`][1]"] # [doc = ""] # [doc = " - `width`: `16` bits"] # [doc = " - `poly`: `0x8005` (reversed: `0xa001`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0xbb3d`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-16-arc"] pub const CRC_16_ARC : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x8005 , init : 0x0 , refin : true , refout : true , xorout : 0x0 , check : 0xbb3d , residue : 0x0 } ;
};
}
