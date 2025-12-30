// Generated macro for CRC_16_MODBUS (const)
macro_rules! Depcrate_algorithmCRC_16_MODBUS {
() => {
// Module: crate::algorithm
// Provides: {"CRC_16_MODBUS"}
// Dependencies: {}
# [doc = " # [`CRC-16/MODBUS`][1]"] # [doc = ""] # [doc = " - `width`: `16` bits"] # [doc = " - `poly`: `0x8005` (reversed: `0xa001`)"] # [doc = " - `init`: `0xffff`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x4b37`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-16-modbus"] pub const CRC_16_MODBUS : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x8005 , init : 0xffff , refin : true , refout : true , xorout : 0x0 , check : 0x4b37 , residue : 0x0 } ;
};
}
