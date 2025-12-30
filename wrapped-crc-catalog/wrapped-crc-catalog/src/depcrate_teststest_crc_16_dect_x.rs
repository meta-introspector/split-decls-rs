// Generated macro for test_CRC_16_DECT_X (function)
macro_rules! Depcrate_teststest_CRC_16_DECT_X {
() => {
// Module: crate::tests
// Provides: {"test_CRC_16_DECT_X"}
// Dependencies: {}
# [test] fn test_CRC_16_DECT_X () { pub const CRC_16_DECT_X : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x0589 , init : 0x0000 , refin : false , refout : false , xorout : 0x0000 , check : 0x007f , residue : 0x0000 } ; assert_eq ! (CRC_16_DECT_X , crate :: algorithm :: CRC_16_DECT_X) ; }
};
}
