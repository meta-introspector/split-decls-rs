// Generated macro for test_CRC_24_LTE_A (function)
macro_rules! Depcrate_teststest_CRC_24_LTE_A {
() => {
// Module: crate::tests
// Provides: {"test_CRC_24_LTE_A"}
// Dependencies: {}
# [test] fn test_CRC_24_LTE_A () { pub const CRC_24_LTE_A : Algorithm < u32 > = Algorithm { width : 24 , poly : 0x864cfb , init : 0x000000 , refin : false , refout : false , xorout : 0x000000 , check : 0xcde703 , residue : 0x000000 } ; assert_eq ! (CRC_24_LTE_A , crate :: algorithm :: CRC_24_LTE_A) ; }
};
}
