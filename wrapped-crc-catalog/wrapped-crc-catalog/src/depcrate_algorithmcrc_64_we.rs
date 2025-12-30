// Generated macro for CRC_64_WE (const)
macro_rules! Depcrate_algorithmCRC_64_WE {
() => {
// Module: crate::algorithm
// Provides: {"CRC_64_WE"}
// Dependencies: {}
# [doc = " # [`CRC-64/WE`][1]"] # [doc = ""] # [doc = " - `width`: `64` bits"] # [doc = " - `poly`: `0x42f0e1eba9ea3693` (reversed: `0xc96c5795d7870f42`)"] # [doc = " - `init`: `0xffffffffffffffff`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0xffffffffffffffff`"] # [doc = " - `check`: `0x62ec59e3f1a4f00a`"] # [doc = " - `residue`: `0xfcacbebd5931a992`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-64-we"] pub const CRC_64_WE : Algorithm < u64 > = Algorithm { width : 64 , poly : 0x42f0e1eba9ea3693 , init : 0xffffffffffffffff , refin : false , refout : false , xorout : 0xffffffffffffffff , check : 0x62ec59e3f1a4f00a , residue : 0xfcacbebd5931a992 } ;
};
}
