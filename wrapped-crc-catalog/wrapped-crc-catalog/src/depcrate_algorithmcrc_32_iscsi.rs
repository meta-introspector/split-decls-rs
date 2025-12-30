// Generated macro for CRC_32_ISCSI (const)
macro_rules! Depcrate_algorithmCRC_32_ISCSI {
() => {
// Module: crate::algorithm
// Provides: {"CRC_32_ISCSI"}
// Dependencies: {}
# [doc = " # [`CRC-32/ISCSI`][1]"] # [doc = ""] # [doc = " - `width`: `32` bits"] # [doc = " - `poly`: `0x1edc6f41` (reversed: `0x82f63b78`)"] # [doc = " - `init`: `0xffffffff`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0xffffffff`"] # [doc = " - `check`: `0xe3069283`"] # [doc = " - `residue`: `0xb798b438`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-32-iscsi"] pub const CRC_32_ISCSI : Algorithm < u32 > = Algorithm { width : 32 , poly : 0x1edc6f41 , init : 0xffffffff , refin : true , refout : true , xorout : 0xffffffff , check : 0xe3069283 , residue : 0xb798b438 } ;
};
}
