// Generated macro for CRC_3_ROHC (const)
macro_rules! Depcrate_algorithmCRC_3_ROHC {
() => {
// Module: crate::algorithm
// Provides: {"CRC_3_ROHC"}
// Dependencies: {}
# [doc = " # [`CRC-3/ROHC`][1]"] # [doc = ""] # [doc = " - `width`: `3` bits"] # [doc = " - `poly`: `0x3` (reversed: `0x6`)"] # [doc = " - `init`: `0x7`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x6`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-3-rohc"] pub const CRC_3_ROHC : Algorithm < u8 > = Algorithm { width : 3 , poly : 0x3 , init : 0x7 , refin : true , refout : true , xorout : 0x0 , check : 0x6 , residue : 0x0 } ;
};
}
