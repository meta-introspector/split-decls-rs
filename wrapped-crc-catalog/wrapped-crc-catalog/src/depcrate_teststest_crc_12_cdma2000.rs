// Generated macro for test_CRC_12_CDMA2000 (function)
macro_rules! Depcrate_teststest_CRC_12_CDMA2000 {
() => {
// Module: crate::tests
// Provides: {"test_CRC_12_CDMA2000"}
// Dependencies: {}
# [test] fn test_CRC_12_CDMA2000 () { pub const CRC_12_CDMA2000 : Algorithm < u16 > = Algorithm { width : 12 , poly : 0xf13 , init : 0xfff , refin : false , refout : false , xorout : 0x000 , check : 0xd4d , residue : 0x000 } ; assert_eq ! (CRC_12_CDMA2000 , crate :: algorithm :: CRC_12_CDMA2000) ; }
};
}
