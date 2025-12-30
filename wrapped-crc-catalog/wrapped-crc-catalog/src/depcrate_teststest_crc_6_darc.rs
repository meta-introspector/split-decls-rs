// Generated macro for test_CRC_6_DARC (function)
macro_rules! Depcrate_teststest_CRC_6_DARC {
() => {
// Module: crate::tests
// Provides: {"test_CRC_6_DARC"}
// Dependencies: {}
# [test] fn test_CRC_6_DARC () { pub const CRC_6_DARC : Algorithm < u8 > = Algorithm { width : 6 , poly : 0x19 , init : 0x00 , refin : true , refout : true , xorout : 0x00 , check : 0x26 , residue : 0x00 } ; assert_eq ! (CRC_6_DARC , crate :: algorithm :: CRC_6_DARC) ; }
};
}
