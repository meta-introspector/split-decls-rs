// Generated macro for CRC_16_XMODEM (const)
macro_rules! Depcrate_algorithmCRC_16_XMODEM {
() => {
// Module: crate::algorithm
// Provides: {"CRC_16_XMODEM"}
// Dependencies: {}
# [doc = " # [`CRC-16/XMODEM`][1]"] # [doc = ""] # [doc = " - `width`: `16` bits"] # [doc = " - `poly`: `0x1021` (reversed: `0x8408`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x31c3`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-16-xmodem"] pub const CRC_16_XMODEM : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x1021 , init : 0x0 , refin : false , refout : false , xorout : 0x0 , check : 0x31c3 , residue : 0x0 } ;
};
}
