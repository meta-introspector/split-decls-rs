// Generated macro for CRC_8_MAXIM_DOW (const)
macro_rules! Depcrate_algorithmCRC_8_MAXIM_DOW {
() => {
// Module: crate::algorithm
// Provides: {"CRC_8_MAXIM_DOW"}
// Dependencies: {}
# [doc = " # [`CRC-8/MAXIM-DOW`][1]"] # [doc = ""] # [doc = " - `width`: `8` bits"] # [doc = " - `poly`: `0x31` (reversed: `0x8c`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0xa1`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-8-maxim-dow"] pub const CRC_8_MAXIM_DOW : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x31 , init : 0x0 , refin : true , refout : true , xorout : 0x0 , check : 0xa1 , residue : 0x0 } ;
};
}
