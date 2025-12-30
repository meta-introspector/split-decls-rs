// Generated macro for CRC_32_ISO_HDLC (const)
macro_rules! Depcrate_algorithmCRC_32_ISO_HDLC {
() => {
// Module: crate::algorithm
// Provides: {"CRC_32_ISO_HDLC"}
// Dependencies: {}
# [doc = " # [`CRC-32/ISO-HDLC`][1]"] # [doc = ""] # [doc = " - `width`: `32` bits"] # [doc = " - `poly`: `0x4c11db7` (reversed: `0xedb88320`)"] # [doc = " - `init`: `0xffffffff`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0xffffffff`"] # [doc = " - `check`: `0xcbf43926`"] # [doc = " - `residue`: `0xdebb20e3`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-32-iso-hdlc"] pub const CRC_32_ISO_HDLC : Algorithm < u32 > = Algorithm { width : 32 , poly : 0x4c11db7 , init : 0xffffffff , refin : true , refout : true , xorout : 0xffffffff , check : 0xcbf43926 , residue : 0xdebb20e3 } ;
};
}
