// Generated macro for test_CRC_8_DARC (function)
macro_rules! Depcrate_teststest_CRC_8_DARC {
() => {
// Module: crate::tests
// Provides: {"test_CRC_8_DARC"}
// Dependencies: {}
# [test] fn test_CRC_8_DARC () { pub const CRC_8_DARC : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x39 , init : 0x00 , refin : true , refout : true , xorout : 0x00 , check : 0x15 , residue : 0x00 } ; assert_eq ! (CRC_8_DARC , crate :: algorithm :: CRC_8_DARC) ; }
};
}
