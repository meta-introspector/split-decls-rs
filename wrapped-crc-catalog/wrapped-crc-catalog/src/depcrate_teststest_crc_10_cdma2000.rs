// Generated macro for test_CRC_10_CDMA2000 (function)
macro_rules! Depcrate_teststest_CRC_10_CDMA2000 {
() => {
// Module: crate::tests
// Provides: {"test_CRC_10_CDMA2000"}
// Dependencies: {}
# [test] fn test_CRC_10_CDMA2000 () { pub const CRC_10_CDMA2000 : Algorithm < u16 > = Algorithm { width : 10 , poly : 0x3d9 , init : 0x3ff , refin : false , refout : false , xorout : 0x000 , check : 0x233 , residue : 0x000 } ; assert_eq ! (CRC_10_CDMA2000 , crate :: algorithm :: CRC_10_CDMA2000) ; }
};
}
