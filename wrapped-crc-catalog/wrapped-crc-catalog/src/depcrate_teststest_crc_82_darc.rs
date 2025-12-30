// Generated macro for test_CRC_82_DARC (function)
macro_rules! Depcrate_teststest_CRC_82_DARC {
() => {
// Module: crate::tests
// Provides: {"test_CRC_82_DARC"}
// Dependencies: {}
# [test] fn test_CRC_82_DARC () { pub const CRC_82_DARC : Algorithm < u128 > = Algorithm { width : 82 , poly : 0x0308c0111011401440411 , init : 0x000000000000000000000 , refin : true , refout : true , xorout : 0x000000000000000000000 , check : 0x09ea83f625023801fd612 , residue : 0x000000000000000000000 } ; assert_eq ! (CRC_82_DARC , crate :: algorithm :: CRC_82_DARC) ; }
};
}
