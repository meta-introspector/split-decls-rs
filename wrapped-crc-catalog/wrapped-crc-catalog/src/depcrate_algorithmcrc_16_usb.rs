// Generated macro for CRC_16_USB (const)
macro_rules! Depcrate_algorithmCRC_16_USB {
() => {
// Module: crate::algorithm
// Provides: {"CRC_16_USB"}
// Dependencies: {}
# [doc = " # [`CRC-16/USB`][1]"] # [doc = ""] # [doc = " - `width`: `16` bits"] # [doc = " - `poly`: `0x8005` (reversed: `0xa001`)"] # [doc = " - `init`: `0xffff`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0xffff`"] # [doc = " - `check`: `0xb4c8`"] # [doc = " - `residue`: `0xb001`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-16-usb"] pub const CRC_16_USB : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x8005 , init : 0xffff , refin : true , refout : true , xorout : 0xffff , check : 0xb4c8 , residue : 0xb001 } ;
};
}
