// Generated macro for test_CRC_8_GSM_A (function)
macro_rules! Depcrate_teststest_CRC_8_GSM_A {
() => {
// Module: crate::tests
// Provides: {"test_CRC_8_GSM_A"}
// Dependencies: {}
# [test] fn test_CRC_8_GSM_A () { pub const CRC_8_GSM_A : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x1d , init : 0x00 , refin : false , refout : false , xorout : 0x00 , check : 0x37 , residue : 0x00 } ; assert_eq ! (CRC_8_GSM_A , crate :: algorithm :: CRC_8_GSM_A) ; }
};
}
