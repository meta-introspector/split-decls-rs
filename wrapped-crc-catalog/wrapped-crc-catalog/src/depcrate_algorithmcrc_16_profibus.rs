// Generated macro for CRC_16_PROFIBUS (const)
macro_rules! Depcrate_algorithmCRC_16_PROFIBUS {
() => {
// Module: crate::algorithm
// Provides: {"CRC_16_PROFIBUS"}
// Dependencies: {}
# [doc = " # [`CRC-16/PROFIBUS`][1]"] # [doc = ""] # [doc = " - `width`: `16` bits"] # [doc = " - `poly`: `0x1dcf` (reversed: `0xf3b8`)"] # [doc = " - `init`: `0xffff`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0xffff`"] # [doc = " - `check`: `0xa819`"] # [doc = " - `residue`: `0xe394`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-16-profibus"] pub const CRC_16_PROFIBUS : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x1dcf , init : 0xffff , refin : false , refout : false , xorout : 0xffff , check : 0xa819 , residue : 0xe394 } ;
};
}
