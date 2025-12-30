// Generated macro for CRC_8_GSM_B (const)
macro_rules! Depcrate_algorithmCRC_8_GSM_B {
() => {
// Module: crate::algorithm
// Provides: {"CRC_8_GSM_B"}
// Dependencies: {}
# [doc = " # [`CRC-8/GSM-B`][1]"] # [doc = ""] # [doc = " - `width`: `8` bits"] # [doc = " - `poly`: `0x49` (reversed: `0x92`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0xff`"] # [doc = " - `check`: `0x94`"] # [doc = " - `residue`: `0x53`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-8-gsm-b"] pub const CRC_8_GSM_B : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x49 , init : 0x0 , refin : false , refout : false , xorout : 0xff , check : 0x94 , residue : 0x53 } ;
};
}
