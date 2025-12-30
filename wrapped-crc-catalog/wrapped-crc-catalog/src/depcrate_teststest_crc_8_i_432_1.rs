// Generated macro for test_CRC_8_I_432_1 (function)
macro_rules! Depcrate_teststest_CRC_8_I_432_1 {
() => {
// Module: crate::tests
// Provides: {"test_CRC_8_I_432_1"}
// Dependencies: {}
# [test] fn test_CRC_8_I_432_1 () { pub const CRC_8_I_432_1 : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x07 , init : 0x00 , refin : false , refout : false , xorout : 0x55 , check : 0xa1 , residue : 0xac } ; assert_eq ! (CRC_8_I_432_1 , crate :: algorithm :: CRC_8_I_432_1) ; }
};
}
