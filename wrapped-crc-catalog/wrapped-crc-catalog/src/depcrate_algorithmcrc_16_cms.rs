// Generated macro for CRC_16_CMS (const)
macro_rules! Depcrate_algorithmCRC_16_CMS {
() => {
// Module: crate::algorithm
// Provides: {"CRC_16_CMS"}
// Dependencies: {}
# [doc = " # [`CRC-16/CMS`][1]"] # [doc = ""] # [doc = " - `width`: `16` bits"] # [doc = " - `poly`: `0x8005` (reversed: `0xa001`)"] # [doc = " - `init`: `0xffff`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0xaee7`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-16-cms"] pub const CRC_16_CMS : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x8005 , init : 0xffff , refin : false , refout : false , xorout : 0x0 , check : 0xaee7 , residue : 0x0 } ;
};
}
