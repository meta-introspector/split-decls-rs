// Generated macro for CRC_8_HITAG (const)
macro_rules! Depcrate_algorithmCRC_8_HITAG {
() => {
// Module: crate::algorithm
// Provides: {"CRC_8_HITAG"}
// Dependencies: {}
# [doc = " # [`CRC-8/HITAG`][1]"] # [doc = ""] # [doc = " - `width`: `8` bits"] # [doc = " - `poly`: `0x1d` (reversed: `0xb8`)"] # [doc = " - `init`: `0xff`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0xb4`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-8-hitag"] pub const CRC_8_HITAG : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x1d , init : 0xff , refin : false , refout : false , xorout : 0x0 , check : 0xb4 , residue : 0x0 } ;
};
}
