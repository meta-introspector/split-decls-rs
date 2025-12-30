// Generated macro for CRC_13_BBC (const)
macro_rules! Depcrate_algorithmCRC_13_BBC {
() => {
// Module: crate::algorithm
// Provides: {"CRC_13_BBC"}
// Dependencies: {}
# [doc = " # [`CRC-13/BBC`][1]"] # [doc = ""] # [doc = " - `width`: `13` bits"] # [doc = " - `poly`: `0x1cf5` (reversed: `0x15e7`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x4fa`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-13-bbc"] pub const CRC_13_BBC : Algorithm < u16 > = Algorithm { width : 13 , poly : 0x1cf5 , init : 0x0 , refin : false , refout : false , xorout : 0x0 , check : 0x4fa , residue : 0x0 } ;
};
}
