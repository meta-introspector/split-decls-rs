// Generated macro for CRC_24_FLEXRAY_B (const)
macro_rules! Depcrate_algorithmCRC_24_FLEXRAY_B {
() => {
// Module: crate::algorithm
// Provides: {"CRC_24_FLEXRAY_B"}
// Dependencies: {}
# [doc = " # [`CRC-24/FLEXRAY-B`][1]"] # [doc = ""] # [doc = " - `width`: `24` bits"] # [doc = " - `poly`: `0x5d6dcb` (reversed: `0xd3b6ba`)"] # [doc = " - `init`: `0xabcdef`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x1f23b8`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-24-flexray-b"] pub const CRC_24_FLEXRAY_B : Algorithm < u32 > = Algorithm { width : 24 , poly : 0x5d6dcb , init : 0xabcdef , refin : false , refout : false , xorout : 0x0 , check : 0x1f23b8 , residue : 0x0 } ;
};
}
