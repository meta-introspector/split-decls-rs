// Generated macro for CRC_16_KERMIT (const)
macro_rules! Depcrate_algorithmCRC_16_KERMIT {
() => {
// Module: crate::algorithm
// Provides: {"CRC_16_KERMIT"}
// Dependencies: {}
# [doc = " # [`CRC-16/KERMIT`][1]"] # [doc = ""] # [doc = " - `width`: `16` bits"] # [doc = " - `poly`: `0x1021` (reversed: `0x8408`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x2189`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-16-kermit"] pub const CRC_16_KERMIT : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x1021 , init : 0x0 , refin : true , refout : true , xorout : 0x0 , check : 0x2189 , residue : 0x0 } ;
};
}
