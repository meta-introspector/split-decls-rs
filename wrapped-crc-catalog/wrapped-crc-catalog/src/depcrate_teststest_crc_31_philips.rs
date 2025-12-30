// Generated macro for test_CRC_31_PHILIPS (function)
macro_rules! Depcrate_teststest_CRC_31_PHILIPS {
() => {
// Module: crate::tests
// Provides: {"test_CRC_31_PHILIPS"}
// Dependencies: {}
# [test] fn test_CRC_31_PHILIPS () { pub const CRC_31_PHILIPS : Algorithm < u32 > = Algorithm { width : 31 , poly : 0x04c11db7 , init : 0x7fffffff , refin : false , refout : false , xorout : 0x7fffffff , check : 0x0ce9e46c , residue : 0x4eaf26f1 } ; assert_eq ! (CRC_31_PHILIPS , crate :: algorithm :: CRC_31_PHILIPS) ; }
};
}
