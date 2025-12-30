// Generated macro for test_CRC_16_PROFIBUS (function)
macro_rules! Depcrate_teststest_CRC_16_PROFIBUS {
() => {
// Module: crate::tests
// Provides: {"test_CRC_16_PROFIBUS"}
// Dependencies: {}
# [test] fn test_CRC_16_PROFIBUS () { pub const CRC_16_PROFIBUS : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x1dcf , init : 0xffff , refin : false , refout : false , xorout : 0xffff , check : 0xa819 , residue : 0xe394 } ; assert_eq ! (CRC_16_PROFIBUS , crate :: algorithm :: CRC_16_PROFIBUS) ; }
};
}
