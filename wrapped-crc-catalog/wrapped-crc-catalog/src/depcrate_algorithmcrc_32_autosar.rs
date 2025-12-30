// Generated macro for CRC_32_AUTOSAR (const)
macro_rules! Depcrate_algorithmCRC_32_AUTOSAR {
() => {
// Module: crate::algorithm
// Provides: {"CRC_32_AUTOSAR"}
// Dependencies: {}
# [doc = " # [`CRC-32/AUTOSAR`][1]"] # [doc = ""] # [doc = " - `width`: `32` bits"] # [doc = " - `poly`: `0xf4acfb13` (reversed: `0xc8df352f`)"] # [doc = " - `init`: `0xffffffff`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0xffffffff`"] # [doc = " - `check`: `0x1697d06a`"] # [doc = " - `residue`: `0x904cddbf`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-32-autosar"] pub const CRC_32_AUTOSAR : Algorithm < u32 > = Algorithm { width : 32 , poly : 0xf4acfb13 , init : 0xffffffff , refin : true , refout : true , xorout : 0xffffffff , check : 0x1697d06a , residue : 0x904cddbf } ;
};
}
