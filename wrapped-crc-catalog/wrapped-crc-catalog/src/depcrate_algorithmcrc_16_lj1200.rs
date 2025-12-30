// Generated macro for CRC_16_LJ1200 (const)
macro_rules! Depcrate_algorithmCRC_16_LJ1200 {
() => {
// Module: crate::algorithm
// Provides: {"CRC_16_LJ1200"}
// Dependencies: {}
# [doc = " # [`CRC-16/LJ1200`][1]"] # [doc = ""] # [doc = " - `width`: `16` bits"] # [doc = " - `poly`: `0x6f63` (reversed: `0xc6f6`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0xbdf4`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-16-lj1200"] pub const CRC_16_LJ1200 : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x6f63 , init : 0x0 , refin : false , refout : false , xorout : 0x0 , check : 0xbdf4 , residue : 0x0 } ;
};
}
