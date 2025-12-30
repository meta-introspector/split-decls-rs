// Generated macro for test_CRC_16_ARC (function)
macro_rules! Depcrate_teststest_CRC_16_ARC {
() => {
// Module: crate::tests
// Provides: {"test_CRC_16_ARC"}
// Dependencies: {}
# [test] fn test_CRC_16_ARC () { pub const CRC_16_ARC : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x8005 , init : 0x0000 , refin : true , refout : true , xorout : 0x0000 , check : 0xbb3d , residue : 0x0000 } ; assert_eq ! (CRC_16_ARC , crate :: algorithm :: CRC_16_ARC) ; }
};
}
