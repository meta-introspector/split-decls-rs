// Generated macro for test_CRC_8_GSM_B (function)
macro_rules! Depcrate_teststest_CRC_8_GSM_B {
() => {
// Module: crate::tests
// Provides: {"test_CRC_8_GSM_B"}
// Dependencies: {}
# [test] fn test_CRC_8_GSM_B () { pub const CRC_8_GSM_B : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x49 , init : 0x00 , refin : false , refout : false , xorout : 0xff , check : 0x94 , residue : 0x53 } ; assert_eq ! (CRC_8_GSM_B , crate :: algorithm :: CRC_8_GSM_B) ; }
};
}
