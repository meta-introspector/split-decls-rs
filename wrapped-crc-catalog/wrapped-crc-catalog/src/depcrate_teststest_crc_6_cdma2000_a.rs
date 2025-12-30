// Generated macro for test_CRC_6_CDMA2000_A (function)
macro_rules! Depcrate_teststest_CRC_6_CDMA2000_A {
() => {
// Module: crate::tests
// Provides: {"test_CRC_6_CDMA2000_A"}
// Dependencies: {}
# [test] fn test_CRC_6_CDMA2000_A () { pub const CRC_6_CDMA2000_A : Algorithm < u8 > = Algorithm { width : 6 , poly : 0x27 , init : 0x3f , refin : false , refout : false , xorout : 0x00 , check : 0x0d , residue : 0x00 } ; assert_eq ! (CRC_6_CDMA2000_A , crate :: algorithm :: CRC_6_CDMA2000_A) ; }
};
}
