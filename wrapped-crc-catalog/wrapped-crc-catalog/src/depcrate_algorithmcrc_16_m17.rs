// Generated macro for CRC_16_M17 (const)
macro_rules! Depcrate_algorithmCRC_16_M17 {
() => {
// Module: crate::algorithm
// Provides: {"CRC_16_M17"}
// Dependencies: {}
# [doc = " # [`CRC-16/M17`][1]"] # [doc = ""] # [doc = " - `width`: `16` bits"] # [doc = " - `poly`: `0x5935` (reversed: `0xac9a`)"] # [doc = " - `init`: `0xffff`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x772b`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-16-m17"] pub const CRC_16_M17 : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x5935 , init : 0xffff , refin : false , refout : false , xorout : 0x0 , check : 0x772b , residue : 0x0 } ;
};
}
