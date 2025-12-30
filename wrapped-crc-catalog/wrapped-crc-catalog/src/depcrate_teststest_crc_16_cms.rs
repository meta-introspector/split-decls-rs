// Generated macro for test_CRC_16_CMS (function)
macro_rules! Depcrate_teststest_CRC_16_CMS {
() => {
// Module: crate::tests
// Provides: {"test_CRC_16_CMS"}
// Dependencies: {}
# [test] fn test_CRC_16_CMS () { pub const CRC_16_CMS : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x8005 , init : 0xffff , refin : false , refout : false , xorout : 0x0000 , check : 0xaee7 , residue : 0x0000 } ; assert_eq ! (CRC_16_CMS , crate :: algorithm :: CRC_16_CMS) ; }
};
}
