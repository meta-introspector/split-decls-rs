// Generated macro for CRC_8_BLUETOOTH (const)
macro_rules! Depcrate_algorithmCRC_8_BLUETOOTH {
() => {
// Module: crate::algorithm
// Provides: {"CRC_8_BLUETOOTH"}
// Dependencies: {}
# [doc = " # [`CRC-8/BLUETOOTH`][1]"] # [doc = ""] # [doc = " - `width`: `8` bits"] # [doc = " - `poly`: `0xa7` (reversed: `0xe5`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x26`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-8-bluetooth"] pub const CRC_8_BLUETOOTH : Algorithm < u8 > = Algorithm { width : 8 , poly : 0xa7 , init : 0x0 , refin : true , refout : true , xorout : 0x0 , check : 0x26 , residue : 0x0 } ;
};
}
