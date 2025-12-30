// Generated macro for CRC_8_I_CODE (const)
macro_rules! Depcrate_algorithmCRC_8_I_CODE {
() => {
// Module: crate::algorithm
// Provides: {"CRC_8_I_CODE"}
// Dependencies: {}
# [doc = " # [`CRC-8/I-CODE`][1]"] # [doc = ""] # [doc = " - `width`: `8` bits"] # [doc = " - `poly`: `0x1d` (reversed: `0xb8`)"] # [doc = " - `init`: `0xfd`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x7e`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-8-i-code"] pub const CRC_8_I_CODE : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x1d , init : 0xfd , refin : false , refout : false , xorout : 0x0 , check : 0x7e , residue : 0x0 } ;
};
}
