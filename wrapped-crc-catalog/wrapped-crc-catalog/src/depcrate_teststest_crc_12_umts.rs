// Generated macro for test_CRC_12_UMTS (function)
macro_rules! Depcrate_teststest_CRC_12_UMTS {
() => {
// Module: crate::tests
// Provides: {"test_CRC_12_UMTS"}
// Dependencies: {}
# [test] fn test_CRC_12_UMTS () { pub const CRC_12_UMTS : Algorithm < u16 > = Algorithm { width : 12 , poly : 0x80f , init : 0x000 , refin : false , refout : true , xorout : 0x000 , check : 0xdaf , residue : 0x000 } ; assert_eq ! (CRC_12_UMTS , crate :: algorithm :: CRC_12_UMTS) ; }
};
}
