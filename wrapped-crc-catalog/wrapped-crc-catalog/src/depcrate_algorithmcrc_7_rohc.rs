// Generated macro for CRC_7_ROHC (const)
macro_rules! Depcrate_algorithmCRC_7_ROHC {
() => {
// Module: crate::algorithm
// Provides: {"CRC_7_ROHC"}
// Dependencies: {}
# [doc = " # [`CRC-7/ROHC`][1]"] # [doc = ""] # [doc = " - `width`: `7` bits"] # [doc = " - `poly`: `0x4f` (reversed: `0x79`)"] # [doc = " - `init`: `0x7f`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x53`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-7-rohc"] pub const CRC_7_ROHC : Algorithm < u8 > = Algorithm { width : 7 , poly : 0x4f , init : 0x7f , refin : true , refout : true , xorout : 0x0 , check : 0x53 , residue : 0x0 } ;
};
}
