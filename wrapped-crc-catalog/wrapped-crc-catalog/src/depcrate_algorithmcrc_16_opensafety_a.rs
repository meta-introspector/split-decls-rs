// Generated macro for CRC_16_OPENSAFETY_A (const)
macro_rules! Depcrate_algorithmCRC_16_OPENSAFETY_A {
() => {
// Module: crate::algorithm
// Provides: {"CRC_16_OPENSAFETY_A"}
// Dependencies: {}
# [doc = " # [`CRC-16/OPENSAFETY-A`][1]"] # [doc = ""] # [doc = " - `width`: `16` bits"] # [doc = " - `poly`: `0x5935` (reversed: `0xac9a`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x5d38`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-16-opensafety-a"] pub const CRC_16_OPENSAFETY_A : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x5935 , init : 0x0 , refin : false , refout : false , xorout : 0x0 , check : 0x5d38 , residue : 0x0 } ;
};
}
