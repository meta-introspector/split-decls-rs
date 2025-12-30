// Generated macro for test_CRC_8_I_CODE (function)
macro_rules! Depcrate_teststest_CRC_8_I_CODE {
() => {
// Module: crate::tests
// Provides: {"test_CRC_8_I_CODE"}
// Dependencies: {}
# [test] fn test_CRC_8_I_CODE () { pub const CRC_8_I_CODE : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x1d , init : 0xfd , refin : false , refout : false , xorout : 0x00 , check : 0x7e , residue : 0x00 } ; assert_eq ! (CRC_8_I_CODE , crate :: algorithm :: CRC_8_I_CODE) ; }
};
}
