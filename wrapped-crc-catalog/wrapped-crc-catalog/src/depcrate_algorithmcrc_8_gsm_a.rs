// Generated macro for CRC_8_GSM_A (const)
macro_rules! Depcrate_algorithmCRC_8_GSM_A {
() => {
// Module: crate::algorithm
// Provides: {"CRC_8_GSM_A"}
// Dependencies: {}
# [doc = " # [`CRC-8/GSM-A`][1]"] # [doc = ""] # [doc = " - `width`: `8` bits"] # [doc = " - `poly`: `0x1d` (reversed: `0xb8`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x37`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-8-gsm-a"] pub const CRC_8_GSM_A : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x1d , init : 0x0 , refin : false , refout : false , xorout : 0x0 , check : 0x37 , residue : 0x0 } ;
};
}
