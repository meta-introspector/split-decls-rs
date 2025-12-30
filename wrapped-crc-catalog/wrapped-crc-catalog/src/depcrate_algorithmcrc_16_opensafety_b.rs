// Generated macro for CRC_16_OPENSAFETY_B (const)
macro_rules! Depcrate_algorithmCRC_16_OPENSAFETY_B {
() => {
// Module: crate::algorithm
// Provides: {"CRC_16_OPENSAFETY_B"}
// Dependencies: {}
# [doc = " # [`CRC-16/OPENSAFETY-B`][1]"] # [doc = ""] # [doc = " - `width`: `16` bits"] # [doc = " - `poly`: `0x755b` (reversed: `0xdaae`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x20fe`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-16-opensafety-b"] pub const CRC_16_OPENSAFETY_B : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x755b , init : 0x0 , refin : false , refout : false , xorout : 0x0 , check : 0x20fe , residue : 0x0 } ;
};
}
