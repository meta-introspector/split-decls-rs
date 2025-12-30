// Generated macro for CRC_15_CAN (const)
macro_rules! Depcrate_algorithmCRC_15_CAN {
() => {
// Module: crate::algorithm
// Provides: {"CRC_15_CAN"}
// Dependencies: {}
# [doc = " # [`CRC-15/CAN`][1]"] # [doc = ""] # [doc = " - `width`: `15` bits"] # [doc = " - `poly`: `0x4599` (reversed: `0x4cd1`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x59e`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-15-can"] pub const CRC_15_CAN : Algorithm < u16 > = Algorithm { width : 15 , poly : 0x4599 , init : 0x0 , refin : false , refout : false , xorout : 0x0 , check : 0x59e , residue : 0x0 } ;
};
}
