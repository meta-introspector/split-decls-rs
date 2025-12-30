// Generated macro for CRC_16_GENIBUS (const)
macro_rules! Depcrate_algorithmCRC_16_GENIBUS {
() => {
// Module: crate::algorithm
// Provides: {"CRC_16_GENIBUS"}
// Dependencies: {}
# [doc = " # [`CRC-16/GENIBUS`][1]"] # [doc = ""] # [doc = " - `width`: `16` bits"] # [doc = " - `poly`: `0x1021` (reversed: `0x8408`)"] # [doc = " - `init`: `0xffff`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0xffff`"] # [doc = " - `check`: `0xd64e`"] # [doc = " - `residue`: `0x1d0f`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-16-genibus"] pub const CRC_16_GENIBUS : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x1021 , init : 0xffff , refin : false , refout : false , xorout : 0xffff , check : 0xd64e , residue : 0x1d0f } ;
};
}
