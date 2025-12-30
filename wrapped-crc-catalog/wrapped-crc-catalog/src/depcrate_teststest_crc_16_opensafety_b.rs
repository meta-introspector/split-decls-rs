// Generated macro for test_CRC_16_OPENSAFETY_B (function)
macro_rules! Depcrate_teststest_CRC_16_OPENSAFETY_B {
() => {
// Module: crate::tests
// Provides: {"test_CRC_16_OPENSAFETY_B"}
// Dependencies: {}
# [test] fn test_CRC_16_OPENSAFETY_B () { pub const CRC_16_OPENSAFETY_B : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x755b , init : 0x0000 , refin : false , refout : false , xorout : 0x0000 , check : 0x20fe , residue : 0x0000 } ; assert_eq ! (CRC_16_OPENSAFETY_B , crate :: algorithm :: CRC_16_OPENSAFETY_B) ; }
};
}
