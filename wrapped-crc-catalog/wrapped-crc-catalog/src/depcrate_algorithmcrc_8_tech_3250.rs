// Generated macro for CRC_8_TECH_3250 (const)
macro_rules! Depcrate_algorithmCRC_8_TECH_3250 {
() => {
// Module: crate::algorithm
// Provides: {"CRC_8_TECH_3250"}
// Dependencies: {}
# [doc = " # [`CRC-8/TECH-3250`][1]"] # [doc = ""] # [doc = " - `width`: `8` bits"] # [doc = " - `poly`: `0x1d` (reversed: `0xb8`)"] # [doc = " - `init`: `0xff`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x97`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-8-tech-3250"] pub const CRC_8_TECH_3250 : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x1d , init : 0xff , refin : true , refout : true , xorout : 0x0 , check : 0x97 , residue : 0x0 } ;
};
}
