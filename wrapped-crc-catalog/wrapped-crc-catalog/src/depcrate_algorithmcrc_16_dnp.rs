// Generated macro for CRC_16_DNP (const)
macro_rules! Depcrate_algorithmCRC_16_DNP {
() => {
// Module: crate::algorithm
// Provides: {"CRC_16_DNP"}
// Dependencies: {}
# [doc = " # [`CRC-16/DNP`][1]"] # [doc = ""] # [doc = " - `width`: `16` bits"] # [doc = " - `poly`: `0x3d65` (reversed: `0xa6bc`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0xffff`"] # [doc = " - `check`: `0xea82`"] # [doc = " - `residue`: `0x66c5`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-16-dnp"] pub const CRC_16_DNP : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x3d65 , init : 0x0 , refin : true , refout : true , xorout : 0xffff , check : 0xea82 , residue : 0x66c5 } ;
};
}
