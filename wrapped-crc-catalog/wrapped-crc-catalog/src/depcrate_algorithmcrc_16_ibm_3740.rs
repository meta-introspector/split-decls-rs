// Generated macro for CRC_16_IBM_3740 (const)
macro_rules! Depcrate_algorithmCRC_16_IBM_3740 {
() => {
// Module: crate::algorithm
// Provides: {"CRC_16_IBM_3740"}
// Dependencies: {}
# [doc = " # [`CRC-16/IBM-3740`][1]"] # [doc = ""] # [doc = " - `width`: `16` bits"] # [doc = " - `poly`: `0x1021` (reversed: `0x8408`)"] # [doc = " - `init`: `0xffff`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x29b1`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-16-ibm-3740"] pub const CRC_16_IBM_3740 : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x1021 , init : 0xffff , refin : false , refout : false , xorout : 0x0 , check : 0x29b1 , residue : 0x0 } ;
};
}
