// Generated macro for test_CRC_8_LTE (function)
macro_rules! Depcrate_teststest_CRC_8_LTE {
() => {
// Module: crate::tests
// Provides: {"test_CRC_8_LTE"}
// Dependencies: {}
# [test] fn test_CRC_8_LTE () { pub const CRC_8_LTE : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x9b , init : 0x00 , refin : false , refout : false , xorout : 0x00 , check : 0xea , residue : 0x00 } ; assert_eq ! (CRC_8_LTE , crate :: algorithm :: CRC_8_LTE) ; }
};
}
