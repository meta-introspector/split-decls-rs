// Generated macro for CRC_10_CDMA2000 (const)
macro_rules! Depcrate_algorithmCRC_10_CDMA2000 {
() => {
// Module: crate::algorithm
// Provides: {"CRC_10_CDMA2000"}
// Dependencies: {}
# [doc = " # [`CRC-10/CDMA2000`][1]"] # [doc = ""] # [doc = " - `width`: `10` bits"] # [doc = " - `poly`: `0x3d9` (reversed: `0x26f`)"] # [doc = " - `init`: `0x3ff`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x233`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-10-cdma2000"] pub const CRC_10_CDMA2000 : Algorithm < u16 > = Algorithm { width : 10 , poly : 0x3d9 , init : 0x3ff , refin : false , refout : false , xorout : 0x0 , check : 0x233 , residue : 0x0 } ;
};
}
