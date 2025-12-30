// Generated macro for test_CRC_14_DARC (function)
macro_rules! Depcrate_teststest_CRC_14_DARC {
() => {
// Module: crate::tests
// Provides: {"test_CRC_14_DARC"}
// Dependencies: {}
# [test] fn test_CRC_14_DARC () { pub const CRC_14_DARC : Algorithm < u16 > = Algorithm { width : 14 , poly : 0x0805 , init : 0x0000 , refin : true , refout : true , xorout : 0x0000 , check : 0x082d , residue : 0x0000 } ; assert_eq ! (CRC_14_DARC , crate :: algorithm :: CRC_14_DARC) ; }
};
}
