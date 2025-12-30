// Generated macro for CRC_32_CD_ROM_EDC (const)
macro_rules! Depcrate_algorithmCRC_32_CD_ROM_EDC {
() => {
// Module: crate::algorithm
// Provides: {"CRC_32_CD_ROM_EDC"}
// Dependencies: {}
# [doc = " # [`CRC-32/CD-ROM-EDC`][1]"] # [doc = ""] # [doc = " - `width`: `32` bits"] # [doc = " - `poly`: `0x8001801b` (reversed: `0xd8018001`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `true`"] # [doc = " - `refout`: `true`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x6ec2edc4`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-32-cd-rom-edc"] pub const CRC_32_CD_ROM_EDC : Algorithm < u32 > = Algorithm { width : 32 , poly : 0x8001801b , init : 0x0 , refin : true , refout : true , xorout : 0x0 , check : 0x6ec2edc4 , residue : 0x0 } ;
};
}
