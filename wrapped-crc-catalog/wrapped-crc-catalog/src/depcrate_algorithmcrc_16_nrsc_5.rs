// Generated macro for CRC_16_NRSC_5 (const)
macro_rules! Depcrate_algorithmCRC_16_NRSC_5 {
() => {
// Module: crate::algorithm
// Provides: {"CRC_16_NRSC_5"}
// Dependencies: {}
# [doc = " # [`CRC-16/NRSC-5`][1]"] # [doc = ""] # [doc = " - `width`: `16` bits"] # [doc = " - `poly`: `0x80b` (reversed: `0xd010`)"] # [doc = " - `init`: `0xffff`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0xa066`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-16-nrsc-5"] pub const CRC_16_NRSC_5 : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x80b , init : 0xffff , refin : true , refout : true , xorout : 0x0 , check : 0xa066 , residue : 0x0 } ;
};
}
