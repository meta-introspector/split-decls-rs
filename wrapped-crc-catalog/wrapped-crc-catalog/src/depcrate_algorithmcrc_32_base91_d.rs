// Generated macro for CRC_32_BASE91_D (const)
macro_rules! Depcrate_algorithmCRC_32_BASE91_D {
() => {
// Module: crate::algorithm
// Provides: {"CRC_32_BASE91_D"}
// Dependencies: {}
# [doc = " # [`CRC-32/BASE91-D`][1]"] # [doc = ""] # [doc = " - `width`: `32` bits"] # [doc = " - `poly`: `0xa833982b` (reversed: `0xd419cc15`)"] # [doc = " - `init`: `0xffffffff`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0xffffffff`"] # [doc = " - `check`: `0x87315576`"] # [doc = " - `residue`: `0x45270551`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-32-base91-d"] pub const CRC_32_BASE91_D : Algorithm < u32 > = Algorithm { width : 32 , poly : 0xa833982b , init : 0xffffffff , refin : true , refout : true , xorout : 0xffffffff , check : 0x87315576 , residue : 0x45270551 } ;
};
}
