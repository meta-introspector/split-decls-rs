// Generated macro for CRC_11_FLEXRAY (const)
macro_rules! Depcrate_algorithmCRC_11_FLEXRAY {
() => {
// Module: crate::algorithm
// Provides: {"CRC_11_FLEXRAY"}
// Dependencies: {}
# [doc = " # [`CRC-11/FLEXRAY`][1]"] # [doc = ""] # [doc = " - `width`: `11` bits"] # [doc = " - `poly`: `0x385` (reversed: `0x50e`)"] # [doc = " - `init`: `0x1a`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x5a3`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-11-flexray"] pub const CRC_11_FLEXRAY : Algorithm < u16 > = Algorithm { width : 11 , poly : 0x385 , init : 0x1a , refin : false , refout : false , xorout : 0x0 , check : 0x5a3 , residue : 0x0 } ;
};
}
