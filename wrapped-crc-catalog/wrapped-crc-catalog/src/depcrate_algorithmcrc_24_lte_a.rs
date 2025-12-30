// Generated macro for CRC_24_LTE_A (const)
macro_rules! Depcrate_algorithmCRC_24_LTE_A {
() => {
// Module: crate::algorithm
// Provides: {"CRC_24_LTE_A"}
// Dependencies: {}
# [doc = " # [`CRC-24/LTE-A`][1]"] # [doc = ""] # [doc = " - `width`: `24` bits"] # [doc = " - `poly`: `0x864cfb` (reversed: `0xdf3261`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0xcde703`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-24-lte-a"] pub const CRC_24_LTE_A : Algorithm < u32 > = Algorithm { width : 24 , poly : 0x864cfb , init : 0x0 , refin : false , refout : false , xorout : 0x0 , check : 0xcde703 , residue : 0x0 } ;
};
}
