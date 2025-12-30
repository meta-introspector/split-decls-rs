// Generated macro for CRC_64_GO_ISO (const)
macro_rules! Depcrate_algorithmCRC_64_GO_ISO {
() => {
// Module: crate::algorithm
// Provides: {"CRC_64_GO_ISO"}
// Dependencies: {}
# [doc = " # [`CRC-64/GO-ISO`][1]"] # [doc = ""] # [doc = " - `width`: `64` bits"] # [doc = " - `poly`: `0x1b` (reversed: `0xd800000000000000`)"] # [doc = " - `init`: `0xffffffffffffffff`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0xffffffffffffffff`"] # [doc = " - `check`: `0xb90956c775a41001`"] # [doc = " - `residue`: `0x5300000000000000`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-64-go-iso"] pub const CRC_64_GO_ISO : Algorithm < u64 > = Algorithm { width : 64 , poly : 0x1b , init : 0xffffffffffffffff , refin : true , refout : true , xorout : 0xffffffffffffffff , check : 0xb90956c775a41001 , residue : 0x5300000000000000 } ;
};
}
