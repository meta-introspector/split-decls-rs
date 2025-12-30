// Generated macro for CRC_30_CDMA (const)
macro_rules! Depcrate_algorithmCRC_30_CDMA {
() => {
// Module: crate::algorithm
// Provides: {"CRC_30_CDMA"}
// Dependencies: {}
# [doc = " # [`CRC-30/CDMA`][1]"] # [doc = ""] # [doc = " - `width`: `30` bits"] # [doc = " - `poly`: `0x2030b9c7` (reversed: `0x38e74301`)"] # [doc = " - `init`: `0x3fffffff`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x3fffffff`"] # [doc = " - `check`: `0x4c34abf`"] # [doc = " - `residue`: `0x34efa55a`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-30-cdma"] pub const CRC_30_CDMA : Algorithm < u32 > = Algorithm { width : 30 , poly : 0x2030b9c7 , init : 0x3fffffff , refin : false , refout : false , xorout : 0x3fffffff , check : 0x4c34abf , residue : 0x34efa55a } ;
};
}
