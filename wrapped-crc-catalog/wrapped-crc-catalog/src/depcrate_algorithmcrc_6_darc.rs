// Generated macro for CRC_6_DARC (const)
macro_rules! Depcrate_algorithmCRC_6_DARC {
() => {
// Module: crate::algorithm
// Provides: {"CRC_6_DARC"}
// Dependencies: {}
# [doc = " # [`CRC-6/DARC`][1]"] # [doc = ""] # [doc = " - `width`: `6` bits"] # [doc = " - `poly`: `0x19` (reversed: `0x26`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x26`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-6-darc"] pub const CRC_6_DARC : Algorithm < u8 > = Algorithm { width : 6 , poly : 0x19 , init : 0x0 , refin : true , refout : true , xorout : 0x0 , check : 0x26 , residue : 0x0 } ;
};
}
