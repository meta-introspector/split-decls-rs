// Generated macro for CRC_24_OPENPGP (const)
macro_rules! Depcrate_algorithmCRC_24_OPENPGP {
() => {
// Module: crate::algorithm
// Provides: {"CRC_24_OPENPGP"}
// Dependencies: {}
# [doc = " # [`CRC-24/OPENPGP`][1]"] # [doc = ""] # [doc = " - `width`: `24` bits"] # [doc = " - `poly`: `0x864cfb` (reversed: `0xdf3261`)"] # [doc = " - `init`: `0xb704ce`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x21cf02`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-24-openpgp"] pub const CRC_24_OPENPGP : Algorithm < u32 > = Algorithm { width : 24 , poly : 0x864cfb , init : 0xb704ce , refin : false , refout : false , xorout : 0x0 , check : 0x21cf02 , residue : 0x0 } ;
};
}
