// Generated macro for test_CRC_6_CDMA2000_B (function)
macro_rules! Depcrate_teststest_CRC_6_CDMA2000_B {
() => {
// Module: crate::tests
// Provides: {"test_CRC_6_CDMA2000_B"}
// Dependencies: {}
# [test] fn test_CRC_6_CDMA2000_B () { pub const CRC_6_CDMA2000_B : Algorithm < u8 > = Algorithm { width : 6 , poly : 0x07 , init : 0x3f , refin : false , refout : false , xorout : 0x00 , check : 0x3b , residue : 0x00 } ; assert_eq ! (CRC_6_CDMA2000_B , crate :: algorithm :: CRC_6_CDMA2000_B) ; }
};
}
