// Generated macro for test_CRC_24_LTE_B (function)
macro_rules! Depcrate_teststest_CRC_24_LTE_B {
() => {
// Module: crate::tests
// Provides: {"test_CRC_24_LTE_B"}
// Dependencies: {}
# [test] fn test_CRC_24_LTE_B () { pub const CRC_24_LTE_B : Algorithm < u32 > = Algorithm { width : 24 , poly : 0x800063 , init : 0x000000 , refin : false , refout : false , xorout : 0x000000 , check : 0x23ef52 , residue : 0x000000 } ; assert_eq ! (CRC_24_LTE_B , crate :: algorithm :: CRC_24_LTE_B) ; }
};
}
