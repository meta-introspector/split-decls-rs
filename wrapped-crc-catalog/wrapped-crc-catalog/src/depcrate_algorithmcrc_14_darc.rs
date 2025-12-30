// Generated macro for CRC_14_DARC (const)
macro_rules! Depcrate_algorithmCRC_14_DARC {
() => {
// Module: crate::algorithm
// Provides: {"CRC_14_DARC"}
// Dependencies: {}
# [doc = " # [`CRC-14/DARC`][1]"] # [doc = ""] # [doc = " - `width`: `14` bits"] # [doc = " - `poly`: `0x805` (reversed: `0x2804`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x82d`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-14-darc"] pub const CRC_14_DARC : Algorithm < u16 > = Algorithm { width : 14 , poly : 0x805 , init : 0x0 , refin : true , refout : true , xorout : 0x0 , check : 0x82d , residue : 0x0 } ;
};
}
