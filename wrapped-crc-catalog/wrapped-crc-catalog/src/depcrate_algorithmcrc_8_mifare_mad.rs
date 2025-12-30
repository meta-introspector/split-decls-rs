// Generated macro for CRC_8_MIFARE_MAD (const)
macro_rules! Depcrate_algorithmCRC_8_MIFARE_MAD {
() => {
// Module: crate::algorithm
// Provides: {"CRC_8_MIFARE_MAD"}
// Dependencies: {}
# [doc = " # [`CRC-8/MIFARE-MAD`][1]"] # [doc = ""] # [doc = " - `width`: `8` bits"] # [doc = " - `poly`: `0x1d` (reversed: `0xb8`)"] # [doc = " - `init`: `0xc7`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x99`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-8-mifare-mad"] pub const CRC_8_MIFARE_MAD : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x1d , init : 0xc7 , refin : false , refout : false , xorout : 0x0 , check : 0x99 , residue : 0x0 } ;
};
}
