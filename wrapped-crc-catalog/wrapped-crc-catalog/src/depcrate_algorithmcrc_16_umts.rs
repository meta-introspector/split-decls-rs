// Generated macro for CRC_16_UMTS (const)
macro_rules! Depcrate_algorithmCRC_16_UMTS {
() => {
// Module: crate::algorithm
// Provides: {"CRC_16_UMTS"}
// Dependencies: {}
# [doc = " # [`CRC-16/UMTS`][1]"] # [doc = ""] # [doc = " - `width`: `16` bits"] # [doc = " - `poly`: `0x8005` (reversed: `0xa001`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0xfee8`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-16-umts"] pub const CRC_16_UMTS : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x8005 , init : 0x0 , refin : false , refout : false , xorout : 0x0 , check : 0xfee8 , residue : 0x0 } ;
};
}
