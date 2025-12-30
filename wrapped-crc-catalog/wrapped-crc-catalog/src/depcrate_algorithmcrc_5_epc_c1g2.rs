// Generated macro for CRC_5_EPC_C1G2 (const)
macro_rules! Depcrate_algorithmCRC_5_EPC_C1G2 {
() => {
// Module: crate::algorithm
// Provides: {"CRC_5_EPC_C1G2"}
// Dependencies: {}
# [doc = " # [`CRC-5/EPC-C1G2`][1]"] # [doc = ""] # [doc = " - `width`: `5` bits"] # [doc = " - `poly`: `0x9` (reversed: `0x12`)"] # [doc = " - `init`: `0x9`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x0`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-5-epc-c1g2"] pub const CRC_5_EPC_C1G2 : Algorithm < u8 > = Algorithm { width : 5 , poly : 0x9 , init : 0x9 , refin : false , refout : false , xorout : 0x0 , check : 0x0 , residue : 0x0 } ;
};
}
