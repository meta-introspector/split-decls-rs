// Generated macro for CRC_10_ATM (const)
macro_rules! Depcrate_algorithmCRC_10_ATM {
() => {
// Module: crate::algorithm
// Provides: {"CRC_10_ATM"}
// Dependencies: {}
# [doc = " # [`CRC-10/ATM`][1]"] # [doc = ""] # [doc = " - `width`: `10` bits"] # [doc = " - `poly`: `0x233` (reversed: `0x331`)"] # [doc = " - `init`: `0x0`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0x199`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-10-atm"] pub const CRC_10_ATM : Algorithm < u16 > = Algorithm { width : 10 , poly : 0x233 , init : 0x0 , refin : false , refout : false , xorout : 0x0 , check : 0x199 , residue : 0x0 } ;
};
}
