// Generated macro for CRC_64_XZ (const)
macro_rules! Depcrate_algorithmCRC_64_XZ {
() => {
// Module: crate::algorithm
// Provides: {"CRC_64_XZ"}
// Dependencies: {}
# [doc = " # [`CRC-64/XZ`][1]"] # [doc = ""] # [doc = " - `width`: `64` bits"] # [doc = " - `poly`: `0x42f0e1eba9ea3693` (reversed: `0xc96c5795d7870f42`)"] # [doc = " - `init`: `0xffffffffffffffff`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0xffffffffffffffff`"] # [doc = " - `check`: `0x995dc9bbdf1939fa`"] # [doc = " - `residue`: `0x49958c9abd7d353f`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-64-xz"] pub const CRC_64_XZ : Algorithm < u64 > = Algorithm { width : 64 , poly : 0x42f0e1eba9ea3693 , init : 0xffffffffffffffff , refin : true , refout : true , xorout : 0xffffffffffffffff , check : 0x995dc9bbdf1939fa , residue : 0x49958c9abd7d353f } ;
};
}
