// Generated macro for test_CRC_16_GENIBUS (function)
macro_rules! Depcrate_teststest_CRC_16_GENIBUS {
() => {
// Module: crate::tests
// Provides: {"test_CRC_16_GENIBUS"}
// Dependencies: {}
# [test] fn test_CRC_16_GENIBUS () { pub const CRC_16_GENIBUS : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x1021 , init : 0xffff , refin : false , refout : false , xorout : 0xffff , check : 0xd64e , residue : 0x1d0f } ; assert_eq ! (CRC_16_GENIBUS , crate :: algorithm :: CRC_16_GENIBUS) ; }
};
}
