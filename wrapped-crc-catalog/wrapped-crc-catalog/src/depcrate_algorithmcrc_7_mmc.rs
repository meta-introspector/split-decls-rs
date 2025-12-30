// Generated macro for CRC_7_MMC (const)
macro_rules! Depcrate_algorithmCRC_7_MMC {
() => {
// Module: crate::algorithm
// Provides: {"CRC_7_MMC"}
// Dependencies: {}
# [doc = " # [`CRC-7/MMC`][1]"] # [doc = ""] # [doc = " - `width`: `7` bits"] # [doc = " - `poly`: `0x9` (reversed: `0x48`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x75`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-7-mmc"] pub const CRC_7_MMC : Algorithm < u8 > = Algorithm { width : 7 , poly : 0x9 , init : 0x0 , refin : false , refout : false , xorout : 0x0 , check : 0x75 , residue : 0x0 } ;
};
}
