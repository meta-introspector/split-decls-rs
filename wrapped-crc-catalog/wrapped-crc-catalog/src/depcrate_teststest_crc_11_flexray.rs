// Generated macro for test_CRC_11_FLEXRAY (function)
macro_rules! Depcrate_teststest_CRC_11_FLEXRAY {
() => {
// Module: crate::tests
// Provides: {"test_CRC_11_FLEXRAY"}
// Dependencies: {}
# [test] fn test_CRC_11_FLEXRAY () { pub const CRC_11_FLEXRAY : Algorithm < u16 > = Algorithm { width : 11 , poly : 0x385 , init : 0x01a , refin : false , refout : false , xorout : 0x000 , check : 0x5a3 , residue : 0x000 } ; assert_eq ! (CRC_11_FLEXRAY , crate :: algorithm :: CRC_11_FLEXRAY) ; }
};
}
