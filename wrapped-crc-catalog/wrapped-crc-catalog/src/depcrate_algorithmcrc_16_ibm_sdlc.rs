// Generated macro for CRC_16_IBM_SDLC (const)
macro_rules! Depcrate_algorithmCRC_16_IBM_SDLC {
() => {
// Module: crate::algorithm
// Provides: {"CRC_16_IBM_SDLC"}
// Dependencies: {}
# [doc = " # [`CRC-16/IBM-SDLC`][1]"] # [doc = ""] # [doc = " - `width`: `16` bits"] # [doc = " - `poly`: `0x1021` (reversed: `0x8408`)"] # [doc = " - `init`: `0xffff`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0xffff`"] # [doc = " - `check`: `0x906e`"] # [doc = " - `residue`: `0xf0b8`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-16-ibm-sdlc"] pub const CRC_16_IBM_SDLC : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x1021 , init : 0xffff , refin : true , refout : true , xorout : 0xffff , check : 0x906e , residue : 0xf0b8 } ;
};
}
