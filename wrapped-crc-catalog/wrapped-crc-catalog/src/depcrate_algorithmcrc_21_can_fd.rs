// Generated macro for CRC_21_CAN_FD (const)
macro_rules! Depcrate_algorithmCRC_21_CAN_FD {
() => {
// Module: crate::algorithm
// Provides: {"CRC_21_CAN_FD"}
// Dependencies: {}
# [doc = " # [`CRC-21/CAN-FD`][1]"] # [doc = ""] # [doc = " - `width`: `21` bits"] # [doc = " - `poly`: `0x102899` (reversed: `0x132281`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0xed841`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-21-can-fd"] pub const CRC_21_CAN_FD : Algorithm < u32 > = Algorithm { width : 21 , poly : 0x102899 , init : 0x0 , refin : false , refout : false , xorout : 0x0 , check : 0xed841 , residue : 0x0 } ;
};
}
