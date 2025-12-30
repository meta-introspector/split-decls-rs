// Generated macro for CRC_64_ECMA_182 (const)
macro_rules! Depcrate_algorithmCRC_64_ECMA_182 {
() => {
// Module: crate::algorithm
// Provides: {"CRC_64_ECMA_182"}
// Dependencies: {}
# [doc = " # [`CRC-64/ECMA-182`][1]"] # [doc = ""] # [doc = " - `width`: `64` bits"] # [doc = " - `poly`: `0x42f0e1eba9ea3693` (reversed: `0xc96c5795d7870f42`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x6c40df5f0b497347`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-64-ecma-182"] pub const CRC_64_ECMA_182 : Algorithm < u64 > = Algorithm { width : 64 , poly : 0x42f0e1eba9ea3693 , init : 0x0 , refin : false , refout : false , xorout : 0x0 , check : 0x6c40df5f0b497347 , residue : 0x0 } ;
};
}
