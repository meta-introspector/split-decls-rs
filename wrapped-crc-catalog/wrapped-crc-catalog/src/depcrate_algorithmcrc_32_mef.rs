// Generated macro for CRC_32_MEF (const)
macro_rules! Depcrate_algorithmCRC_32_MEF {
() => {
// Module: crate::algorithm
// Provides: {"CRC_32_MEF"}
// Dependencies: {}
# [doc = " # [`CRC-32/MEF`][1]"] # [doc = ""] # [doc = " - `width`: `32` bits"] # [doc = " - `poly`: `0x741b8cd7` (reversed: `0xeb31d82e`)"] # [doc = " - `init`: `0xffffffff`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0xd2c22f51`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-32-mef"] pub const CRC_32_MEF : Algorithm < u32 > = Algorithm { width : 32 , poly : 0x741b8cd7 , init : 0xffffffff , refin : true , refout : true , xorout : 0x0 , check : 0xd2c22f51 , residue : 0x0 } ;
};
}
