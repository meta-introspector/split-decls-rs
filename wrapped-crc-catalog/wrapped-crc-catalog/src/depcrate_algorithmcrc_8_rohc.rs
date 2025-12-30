// Generated macro for CRC_8_ROHC (const)
macro_rules! Depcrate_algorithmCRC_8_ROHC {
() => {
// Module: crate::algorithm
// Provides: {"CRC_8_ROHC"}
// Dependencies: {}
# [doc = " # [`CRC-8/ROHC`][1]"] # [doc = ""] # [doc = " - `width`: `8` bits"] # [doc = " - `poly`: `0x7` (reversed: `0xe0`)"] # [doc = " - `init`: `0xff`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0xd0`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-8-rohc"] pub const CRC_8_ROHC : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x7 , init : 0xff , refin : true , refout : true , xorout : 0x0 , check : 0xd0 , residue : 0x0 } ;
};
}
