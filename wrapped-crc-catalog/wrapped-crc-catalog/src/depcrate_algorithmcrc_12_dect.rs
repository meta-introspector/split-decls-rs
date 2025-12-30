// Generated macro for CRC_12_DECT (const)
macro_rules! Depcrate_algorithmCRC_12_DECT {
() => {
// Module: crate::algorithm
// Provides: {"CRC_12_DECT"}
// Dependencies: {}
# [doc = " # [`CRC-12/DECT`][1]"] # [doc = ""] # [doc = " - `width`: `12` bits"] # [doc = " - `poly`: `0x80f` (reversed: `0xf01`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0xf5b`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-12-dect"] pub const CRC_12_DECT : Algorithm < u16 > = Algorithm { width : 12 , poly : 0x80f , init : 0x0 , refin : false , refout : false , xorout : 0x0 , check : 0xf5b , residue : 0x0 } ;
};
}
