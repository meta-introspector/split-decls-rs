// Generated macro for CRC_16_DDS_110 (const)
macro_rules! Depcrate_algorithmCRC_16_DDS_110 {
() => {
// Module: crate::algorithm
// Provides: {"CRC_16_DDS_110"}
// Dependencies: {}
# [doc = " # [`CRC-16/DDS-110`][1]"] # [doc = ""] # [doc = " - `width`: `16` bits"] # [doc = " - `poly`: `0x8005` (reversed: `0xa001`)"] # [doc = " - `init`: `0x800d`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x9ecf`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-16-dds-110"] pub const CRC_16_DDS_110 : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x8005 , init : 0x800d , refin : false , refout : false , xorout : 0x0 , check : 0x9ecf , residue : 0x0 } ;
};
}
