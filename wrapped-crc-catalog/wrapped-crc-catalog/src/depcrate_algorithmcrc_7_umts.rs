// Generated macro for CRC_7_UMTS (const)
macro_rules! Depcrate_algorithmCRC_7_UMTS {
() => {
// Module: crate::algorithm
// Provides: {"CRC_7_UMTS"}
// Dependencies: {}
# [doc = " # [`CRC-7/UMTS`][1]"] # [doc = ""] # [doc = " - `width`: `7` bits"] # [doc = " - `poly`: `0x45` (reversed: `0x51`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x61`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-7-umts"] pub const CRC_7_UMTS : Algorithm < u8 > = Algorithm { width : 7 , poly : 0x45 , init : 0x0 , refin : false , refout : false , xorout : 0x0 , check : 0x61 , residue : 0x0 } ;
};
}
