// Generated macro for CRC_5_USB (const)
macro_rules! Depcrate_algorithmCRC_5_USB {
() => {
// Module: crate::algorithm
// Provides: {"CRC_5_USB"}
// Dependencies: {}
# [doc = " # [`CRC-5/USB`][1]"] # [doc = ""] # [doc = " - `width`: `5` bits"] # [doc = " - `poly`: `0x5` (reversed: `0x14`)"] # [doc = " - `init`: `0x1f`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0x1f`"] # [doc = " - `check`: `0x19`"] # [doc = " - `residue`: `0x6`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-5-usb"] pub const CRC_5_USB : Algorithm < u8 > = Algorithm { width : 5 , poly : 0x5 , init : 0x1f , refin : true , refout : true , xorout : 0x1f , check : 0x19 , residue : 0x6 } ;
};
}
