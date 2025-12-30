// Generated macro for CRC_8_I_432_1 (const)
macro_rules! Depcrate_algorithmCRC_8_I_432_1 {
() => {
// Module: crate::algorithm
// Provides: {"CRC_8_I_432_1"}
// Dependencies: {}
# [doc = " # [`CRC-8/I-432-1`][1]"] # [doc = ""] # [doc = " - `width`: `8` bits"] # [doc = " - `poly`: `0x7` (reversed: `0xe0`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x55`"] # [doc = " - `check`: `0xa1`"] # [doc = " - `residue`: `0xac`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-8-i-432-1"] pub const CRC_8_I_432_1 : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x7 , init : 0x0 , refin : false , refout : false , xorout : 0x55 , check : 0xa1 , residue : 0xac } ;
};
}
