// Generated macro for CRC_16_TMS37157 (const)
macro_rules! Depcrate_algorithmCRC_16_TMS37157 {
() => {
// Module: crate::algorithm
// Provides: {"CRC_16_TMS37157"}
// Dependencies: {}
# [doc = " # [`CRC-16/TMS37157`][1]"] # [doc = ""] # [doc = " - `width`: `16` bits"] # [doc = " - `poly`: `0x1021` (reversed: `0x8408`)"] # [doc = " - `init`: `0x89ec`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x26b1`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-16-tms37157"] pub const CRC_16_TMS37157 : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x1021 , init : 0x89ec , refin : true , refout : true , xorout : 0x0 , check : 0x26b1 , residue : 0x0 } ;
};
}
