// Generated macro for test_CRC_16_OPENSAFETY_A (function)
macro_rules! Depcrate_teststest_CRC_16_OPENSAFETY_A {
() => {
// Module: crate::tests
// Provides: {"test_CRC_16_OPENSAFETY_A"}
// Dependencies: {}
# [test] fn test_CRC_16_OPENSAFETY_A () { pub const CRC_16_OPENSAFETY_A : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x5935 , init : 0x0000 , refin : false , refout : false , xorout : 0x0000 , check : 0x5d38 , residue : 0x0000 } ; assert_eq ! (CRC_16_OPENSAFETY_A , crate :: algorithm :: CRC_16_OPENSAFETY_A) ; }
};
}
