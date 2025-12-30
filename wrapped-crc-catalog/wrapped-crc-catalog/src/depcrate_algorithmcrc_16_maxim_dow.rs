// Generated macro for CRC_16_MAXIM_DOW (const)
macro_rules! Depcrate_algorithmCRC_16_MAXIM_DOW {
() => {
// Module: crate::algorithm
// Provides: {"CRC_16_MAXIM_DOW"}
// Dependencies: {}
# [doc = " # [`CRC-16/MAXIM-DOW`][1]"] # [doc = ""] # [doc = " - `width`: `16` bits"] # [doc = " - `poly`: `0x8005` (reversed: `0xa001`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0xffff`"] # [doc = " - `check`: `0x44c2`"] # [doc = " - `residue`: `0xb001`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-16-maxim-dow"] pub const CRC_16_MAXIM_DOW : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x8005 , init : 0x0 , refin : true , refout : true , xorout : 0xffff , check : 0x44c2 , residue : 0xb001 } ;
};
}
