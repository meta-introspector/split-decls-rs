// Generated macro for CRC_82_DARC (const)
macro_rules! Depcrate_algorithmCRC_82_DARC {
() => {
// Module: crate::algorithm
// Provides: {"CRC_82_DARC"}
// Dependencies: {}
# [doc = " # [`CRC-82/DARC`][1]"] # [doc = ""] # [doc = " - `width`: `82` bits"] # [doc = " - `poly`: `0x308c0111011401440411` (reversed: `0x220808a00a2022200c430`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x9ea83f625023801fd612`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-82-darc"] pub const CRC_82_DARC : Algorithm < u128 > = Algorithm { width : 82 , poly : 0x308c0111011401440411 , init : 0x0 , refin : true , refout : true , xorout : 0x0 , check : 0x9ea83f625023801fd612 , residue : 0x0 } ;
};
}
