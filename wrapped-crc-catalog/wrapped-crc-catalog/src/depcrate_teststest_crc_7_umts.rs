// Generated macro for test_CRC_7_UMTS (function)
macro_rules! Depcrate_teststest_CRC_7_UMTS {
() => {
// Module: crate::tests
// Provides: {"test_CRC_7_UMTS"}
// Dependencies: {}
# [test] fn test_CRC_7_UMTS () { pub const CRC_7_UMTS : Algorithm < u8 > = Algorithm { width : 7 , poly : 0x45 , init : 0x00 , refin : false , refout : false , xorout : 0x00 , check : 0x61 , residue : 0x00 } ; assert_eq ! (CRC_7_UMTS , crate :: algorithm :: CRC_7_UMTS) ; }
};
}
