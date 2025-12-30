// Generated macro for test_CRC_16_CDMA2000 (function)
macro_rules! Depcrate_teststest_CRC_16_CDMA2000 {
() => {
// Module: crate::tests
// Provides: {"test_CRC_16_CDMA2000"}
// Dependencies: {}
# [test] fn test_CRC_16_CDMA2000 () { pub const CRC_16_CDMA2000 : Algorithm < u16 > = Algorithm { width : 16 , poly : 0xc867 , init : 0xffff , refin : false , refout : false , xorout : 0x0000 , check : 0x4c06 , residue : 0x0000 } ; assert_eq ! (CRC_16_CDMA2000 , crate :: algorithm :: CRC_16_CDMA2000) ; }
};
}
