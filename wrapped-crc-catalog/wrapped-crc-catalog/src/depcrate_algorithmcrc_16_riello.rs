// Generated macro for CRC_16_RIELLO (const)
macro_rules! Depcrate_algorithmCRC_16_RIELLO {
() => {
// Module: crate::algorithm
// Provides: {"CRC_16_RIELLO"}
// Dependencies: {}
# [doc = " # [`CRC-16/RIELLO`][1]"] # [doc = ""] # [doc = " - `width`: `16` bits"] # [doc = " - `poly`: `0x1021` (reversed: `0x8408`)"] # [doc = " - `init`: `0xb2aa`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x63d0`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-16-riello"] pub const CRC_16_RIELLO : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x1021 , init : 0xb2aa , refin : true , refout : true , xorout : 0x0 , check : 0x63d0 , residue : 0x0 } ;
};
}
