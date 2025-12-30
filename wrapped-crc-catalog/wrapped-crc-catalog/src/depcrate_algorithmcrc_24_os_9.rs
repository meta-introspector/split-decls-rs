// Generated macro for CRC_24_OS_9 (const)
macro_rules! Depcrate_algorithmCRC_24_OS_9 {
() => {
// Module: crate::algorithm
// Provides: {"CRC_24_OS_9"}
// Dependencies: {}
# [doc = " # [`CRC-24/OS-9`][1]"] # [doc = ""] # [doc = " - `width`: `24` bits"] # [doc = " - `poly`: `0x800063` (reversed: `0xc60001`)"] # [doc = " - `init`: `0xffffff`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0xffffff`"] # [doc = " - `check`: `0x200fa5`"] # [doc = " - `residue`: `0x800fe3`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-24-os-9"] pub const CRC_24_OS_9 : Algorithm < u32 > = Algorithm { width : 24 , poly : 0x800063 , init : 0xffffff , refin : false , refout : false , xorout : 0xffffff , check : 0x200fa5 , residue : 0x800fe3 } ;
};
}
