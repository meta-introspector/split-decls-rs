// Generated macro for CRC_12_UMTS (const)
macro_rules! Depcrate_algorithmCRC_12_UMTS {
() => {
// Module: crate::algorithm
// Provides: {"CRC_12_UMTS"}
// Dependencies: {}
# [doc = " # [`CRC-12/UMTS`][1]"] # [doc = ""] # [doc = " - `width`: `12` bits"] # [doc = " - `poly`: `0x80f` (reversed: `0xf01`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0xdaf`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-12-umts"] pub const CRC_12_UMTS : Algorithm < u16 > = Algorithm { width : 12 , poly : 0x80f , init : 0x0 , refin : false , refout : true , xorout : 0x0 , check : 0xdaf , residue : 0x0 } ;
};
}
