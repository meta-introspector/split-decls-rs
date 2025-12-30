// Generated macro for test_CRC_11_UMTS (function)
macro_rules! Depcrate_teststest_CRC_11_UMTS {
() => {
// Module: crate::tests
// Provides: {"test_CRC_11_UMTS"}
// Dependencies: {}
# [test] fn test_CRC_11_UMTS () { pub const CRC_11_UMTS : Algorithm < u16 > = Algorithm { width : 11 , poly : 0x307 , init : 0x000 , refin : false , refout : false , xorout : 0x000 , check : 0x061 , residue : 0x000 } ; assert_eq ! (CRC_11_UMTS , crate :: algorithm :: CRC_11_UMTS) ; }
};
}
