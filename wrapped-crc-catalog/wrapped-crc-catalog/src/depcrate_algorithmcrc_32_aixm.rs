// Generated macro for CRC_32_AIXM (const)
macro_rules! Depcrate_algorithmCRC_32_AIXM {
() => {
// Module: crate::algorithm
// Provides: {"CRC_32_AIXM"}
// Dependencies: {}
# [doc = " # [`CRC-32/AIXM`][1]"] # [doc = ""] # [doc = " - `width`: `32` bits"] # [doc = " - `poly`: `0x814141ab` (reversed: `0xd5828281`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x3010bf7f`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-32-aixm"] pub const CRC_32_AIXM : Algorithm < u32 > = Algorithm { width : 32 , poly : 0x814141ab , init : 0x0 , refin : false , refout : false , xorout : 0x0 , check : 0x3010bf7f , residue : 0x0 } ;
};
}
