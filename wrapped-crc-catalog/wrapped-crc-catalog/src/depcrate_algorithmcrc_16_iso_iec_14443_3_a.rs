// Generated macro for CRC_16_ISO_IEC_14443_3_A (const)
macro_rules! Depcrate_algorithmCRC_16_ISO_IEC_14443_3_A {
() => {
// Module: crate::algorithm
// Provides: {"CRC_16_ISO_IEC_14443_3_A"}
// Dependencies: {}
# [doc = " # [`CRC-16/ISO-IEC-14443-3-A`][1]"] # [doc = ""] # [doc = " - `width`: `16` bits"] # [doc = " - `poly`: `0x1021` (reversed: `0x8408`)"] # [doc = " - `init`: `0xc6c6`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0xbf05`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-16-iso-iec-14443-3-a"] pub const CRC_16_ISO_IEC_14443_3_A : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x1021 , init : 0xc6c6 , refin : true , refout : true , xorout : 0x0 , check : 0xbf05 , residue : 0x0 } ;
};
}
