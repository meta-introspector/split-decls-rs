// Generated macro for test_CRC_16_EN_13757 (function)
macro_rules! Depcrate_teststest_CRC_16_EN_13757 {
() => {
// Module: crate::tests
// Provides: {"test_CRC_16_EN_13757"}
// Dependencies: {}
# [test] fn test_CRC_16_EN_13757 () { pub const CRC_16_EN_13757 : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x3d65 , init : 0x0000 , refin : false , refout : false , xorout : 0xffff , check : 0xc2b7 , residue : 0xa366 } ; assert_eq ! (CRC_16_EN_13757 , crate :: algorithm :: CRC_16_EN_13757) ; }
};
}
