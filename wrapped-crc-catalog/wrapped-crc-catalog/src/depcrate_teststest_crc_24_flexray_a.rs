// Generated macro for test_CRC_24_FLEXRAY_A (function)
macro_rules! Depcrate_teststest_CRC_24_FLEXRAY_A {
() => {
// Module: crate::tests
// Provides: {"test_CRC_24_FLEXRAY_A"}
// Dependencies: {}
# [test] fn test_CRC_24_FLEXRAY_A () { pub const CRC_24_FLEXRAY_A : Algorithm < u32 > = Algorithm { width : 24 , poly : 0x5d6dcb , init : 0xfedcba , refin : false , refout : false , xorout : 0x000000 , check : 0x7979bd , residue : 0x000000 } ; assert_eq ! (CRC_24_FLEXRAY_A , crate :: algorithm :: CRC_24_FLEXRAY_A) ; }
};
}
