// Generated macro for CRC_24_BLE (const)
macro_rules! Depcrate_algorithmCRC_24_BLE {
() => {
// Module: crate::algorithm
// Provides: {"CRC_24_BLE"}
// Dependencies: {}
# [doc = " # [`CRC-24/BLE`][1]"] # [doc = ""] # [doc = " - `width`: `24` bits"] # [doc = " - `poly`: `0x65b` (reversed: `0xda6000`)"] # [doc = " - `init`: `0x555555`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0xc25a56`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-24-ble"] pub const CRC_24_BLE : Algorithm < u32 > = Algorithm { width : 24 , poly : 0x65b , init : 0x555555 , refin : true , refout : true , xorout : 0x0 , check : 0xc25a56 , residue : 0x0 } ;
};
}
