// Generated macro for test_CRC_8_WCDMA (function)
macro_rules! Depcrate_teststest_CRC_8_WCDMA {
() => {
// Module: crate::tests
// Provides: {"test_CRC_8_WCDMA"}
// Dependencies: {}
# [test] fn test_CRC_8_WCDMA () { pub const CRC_8_WCDMA : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x9b , init : 0x00 , refin : true , refout : true , xorout : 0x00 , check : 0x25 , residue : 0x00 } ; assert_eq ! (CRC_8_WCDMA , crate :: algorithm :: CRC_8_WCDMA) ; }
};
}
