// Generated macro for CRC_11_UMTS (const)
macro_rules! Depcrate_algorithmCRC_11_UMTS {
() => {
// Module: crate::algorithm
// Provides: {"CRC_11_UMTS"}
// Dependencies: {}
# [doc = " # [`CRC-11/UMTS`][1]"] # [doc = ""] # [doc = " - `width`: `11` bits"] # [doc = " - `poly`: `0x307` (reversed: `0x706`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x61`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-11-umts"] pub const CRC_11_UMTS : Algorithm < u16 > = Algorithm { width : 11 , poly : 0x307 , init : 0x0 , refin : false , refout : false , xorout : 0x0 , check : 0x61 , residue : 0x0 } ;
};
}
