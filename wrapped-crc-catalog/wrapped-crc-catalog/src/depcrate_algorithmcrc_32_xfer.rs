// Generated macro for CRC_32_XFER (const)
macro_rules! Depcrate_algorithmCRC_32_XFER {
() => {
// Module: crate::algorithm
// Provides: {"CRC_32_XFER"}
// Dependencies: {}
# [doc = " # [`CRC-32/XFER`][1]"] # [doc = ""] # [doc = " - `width`: `32` bits"] # [doc = " - `poly`: `0xaf` (reversed: `0xf5000000`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0xbd0be338`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-32-xfer"] pub const CRC_32_XFER : Algorithm < u32 > = Algorithm { width : 32 , poly : 0xaf , init : 0x0 , refin : false , refout : false , xorout : 0x0 , check : 0xbd0be338 , residue : 0x0 } ;
};
}
