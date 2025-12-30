// Generated macro for test_CRC_8_CDMA2000 (function)
macro_rules! Depcrate_teststest_CRC_8_CDMA2000 {
() => {
// Module: crate::tests
// Provides: {"test_CRC_8_CDMA2000"}
// Dependencies: {}
# [test] fn test_CRC_8_CDMA2000 () { pub const CRC_8_CDMA2000 : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x9b , init : 0xff , refin : false , refout : false , xorout : 0x00 , check : 0xda , residue : 0x00 } ; assert_eq ! (CRC_8_CDMA2000 , crate :: algorithm :: CRC_8_CDMA2000) ; }
};
}
