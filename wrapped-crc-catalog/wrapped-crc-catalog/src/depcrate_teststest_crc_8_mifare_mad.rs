// Generated macro for test_CRC_8_MIFARE_MAD (function)
macro_rules! Depcrate_teststest_CRC_8_MIFARE_MAD {
() => {
// Module: crate::tests
// Provides: {"test_CRC_8_MIFARE_MAD"}
// Dependencies: {}
# [test] fn test_CRC_8_MIFARE_MAD () { pub const CRC_8_MIFARE_MAD : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x1d , init : 0xc7 , refin : false , refout : false , xorout : 0x00 , check : 0x99 , residue : 0x00 } ; assert_eq ! (CRC_8_MIFARE_MAD , crate :: algorithm :: CRC_8_MIFARE_MAD) ; }
};
}
