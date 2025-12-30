// Generated macro for CRC_16_MCRF4XX (const)
macro_rules! Depcrate_algorithmCRC_16_MCRF4XX {
() => {
// Module: crate::algorithm
// Provides: {"CRC_16_MCRF4XX"}
// Dependencies: {}
# [doc = " # [`CRC-16/MCRF4XX`][1]"] # [doc = ""] # [doc = " - `width`: `16` bits"] # [doc = " - `poly`: `0x1021` (reversed: `0x8408`)"] # [doc = " - `init`: `0xffff`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x6f91`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-16-mcrf4xx"] pub const CRC_16_MCRF4XX : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x1021 , init : 0xffff , refin : true , refout : true , xorout : 0x0 , check : 0x6f91 , residue : 0x0 } ;
};
}
