// Generated macro for CRC_8_SAE_J1850 (const)
macro_rules! Depcrate_algorithmCRC_8_SAE_J1850 {
() => {
// Module: crate::algorithm
// Provides: {"CRC_8_SAE_J1850"}
// Dependencies: {}
# [doc = " # [`CRC-8/SAE-J1850`][1]"] # [doc = ""] # [doc = " - `width`: `8` bits"] # [doc = " - `poly`: `0x1d` (reversed: `0xb8`)"] # [doc = " - `init`: `0xff`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0xff`"] # [doc = " - `check`: `0x4b`"] # [doc = " - `residue`: `0xc4`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-8-sae-j1850"] pub const CRC_8_SAE_J1850 : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x1d , init : 0xff , refin : false , refout : false , xorout : 0xff , check : 0x4b , residue : 0xc4 } ;
};
}
