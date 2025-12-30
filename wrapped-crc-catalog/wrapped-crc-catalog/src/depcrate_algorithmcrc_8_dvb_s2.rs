// Generated macro for CRC_8_DVB_S2 (const)
macro_rules! Depcrate_algorithmCRC_8_DVB_S2 {
() => {
// Module: crate::algorithm
// Provides: {"CRC_8_DVB_S2"}
// Dependencies: {}
# [doc = " # [`CRC-8/DVB-S2`][1]"] # [doc = ""] # [doc = " - `width`: `8` bits"] # [doc = " - `poly`: `0xd5` (reversed: `0xab`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0xbc`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-8-dvb-s2"] pub const CRC_8_DVB_S2 : Algorithm < u8 > = Algorithm { width : 8 , poly : 0xd5 , init : 0x0 , refin : false , refout : false , xorout : 0x0 , check : 0xbc , residue : 0x0 } ;
};
}
