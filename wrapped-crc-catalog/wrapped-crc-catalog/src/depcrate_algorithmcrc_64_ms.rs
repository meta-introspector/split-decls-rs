// Generated macro for CRC_64_MS (const)
macro_rules! Depcrate_algorithmCRC_64_MS {
() => {
// Module: crate::algorithm
// Provides: {"CRC_64_MS"}
// Dependencies: {}
# [doc = " # [`CRC-64/MS`][1]"] # [doc = ""] # [doc = " - `width`: `64` bits"] # [doc = " - `poly`: `0x259c84cba6426349` (reversed: `0x92c64265d32139a4`)"] # [doc = " - `init`: `0xffffffffffffffff`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x75d4b74f024eceea`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-64-ms"] pub const CRC_64_MS : Algorithm < u64 > = Algorithm { width : 64 , poly : 0x259c84cba6426349 , init : 0xffffffffffffffff , refin : true , refout : true , xorout : 0x0 , check : 0x75d4b74f024eceea , residue : 0x0 } ;
};
}
