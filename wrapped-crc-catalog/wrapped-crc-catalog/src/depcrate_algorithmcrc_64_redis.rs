// Generated macro for CRC_64_REDIS (const)
macro_rules! Depcrate_algorithmCRC_64_REDIS {
() => {
// Module: crate::algorithm
// Provides: {"CRC_64_REDIS"}
// Dependencies: {}
# [doc = " # [`CRC-64/REDIS`][1]"] # [doc = ""] # [doc = " - `width`: `64` bits"] # [doc = " - `poly`: `0xad93d23594c935a9` (reversed: `0x95ac9329ac4bc9b5`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0xe9c6d914c4b8d9ca`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-64-redis"] pub const CRC_64_REDIS : Algorithm < u64 > = Algorithm { width : 64 , poly : 0xad93d23594c935a9 , init : 0x0 , refin : true , refout : true , xorout : 0x0 , check : 0xe9c6d914c4b8d9ca , residue : 0x0 } ;
};
}
