// Generated macro for test_CRC_24_FLEXRAY_B (function)
macro_rules! Depcrate_teststest_CRC_24_FLEXRAY_B {
() => {
// Module: crate::tests
// Provides: {"test_CRC_24_FLEXRAY_B"}
// Dependencies: {}
# [test] fn test_CRC_24_FLEXRAY_B () { pub const CRC_24_FLEXRAY_B : Algorithm < u32 > = Algorithm { width : 24 , poly : 0x5d6dcb , init : 0xabcdef , refin : false , refout : false , xorout : 0x000000 , check : 0x1f23b8 , residue : 0x000000 } ; assert_eq ! (CRC_24_FLEXRAY_B , crate :: algorithm :: CRC_24_FLEXRAY_B) ; }
};
}
