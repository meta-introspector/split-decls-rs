// Generated macro for test_CRC_16_UMTS (function)
macro_rules! Depcrate_teststest_CRC_16_UMTS {
() => {
// Module: crate::tests
// Provides: {"test_CRC_16_UMTS"}
// Dependencies: {}
# [test] fn test_CRC_16_UMTS () { pub const CRC_16_UMTS : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x8005 , init : 0x0000 , refin : false , refout : false , xorout : 0x0000 , check : 0xfee8 , residue : 0x0000 } ; assert_eq ! (CRC_16_UMTS , crate :: algorithm :: CRC_16_UMTS) ; }
};
}
