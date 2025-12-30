// Generated macro for test_CRC_8_SMBUS (function)
macro_rules! Depcrate_teststest_CRC_8_SMBUS {
() => {
// Module: crate::tests
// Provides: {"test_CRC_8_SMBUS"}
// Dependencies: {}
# [test] fn test_CRC_8_SMBUS () { pub const CRC_8_SMBUS : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x07 , init : 0x00 , refin : false , refout : false , xorout : 0x00 , check : 0xf4 , residue : 0x00 } ; assert_eq ! (CRC_8_SMBUS , crate :: algorithm :: CRC_8_SMBUS) ; }
};
}
