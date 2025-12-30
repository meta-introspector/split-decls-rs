// Generated macro for CRC_17_CAN_FD (const)
macro_rules! Depcrate_algorithmCRC_17_CAN_FD {
() => {
// Module: crate::algorithm
// Provides: {"CRC_17_CAN_FD"}
// Dependencies: {}
# [doc = " # [`CRC-17/CAN-FD`][1]"] # [doc = ""] # [doc = " - `width`: `17` bits"] # [doc = " - `poly`: `0x1685b` (reversed: `0x1b42d`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x4f03`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-17-can-fd"] pub const CRC_17_CAN_FD : Algorithm < u32 > = Algorithm { width : 17 , poly : 0x1685b , init : 0x0 , refin : false , refout : false , xorout : 0x0 , check : 0x4f03 , residue : 0x0 } ;
};
}
