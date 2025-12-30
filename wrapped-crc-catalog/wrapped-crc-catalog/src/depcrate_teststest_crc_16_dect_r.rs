// Generated macro for test_CRC_16_DECT_R (function)
macro_rules! Depcrate_teststest_CRC_16_DECT_R {
() => {
// Module: crate::tests
// Provides: {"test_CRC_16_DECT_R"}
// Dependencies: {}
# [test] fn test_CRC_16_DECT_R () { pub const CRC_16_DECT_R : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x0589 , init : 0x0000 , refin : false , refout : false , xorout : 0x0001 , check : 0x007e , residue : 0x0589 } ; assert_eq ! (CRC_16_DECT_R , crate :: algorithm :: CRC_16_DECT_R) ; }
};
}
