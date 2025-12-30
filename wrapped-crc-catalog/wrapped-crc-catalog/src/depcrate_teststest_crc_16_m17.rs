// Generated macro for test_CRC_16_M17 (function)
macro_rules! Depcrate_teststest_CRC_16_M17 {
() => {
// Module: crate::tests
// Provides: {"test_CRC_16_M17"}
// Dependencies: {}
# [test] fn test_CRC_16_M17 () { pub const CRC_16_M17 : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x5935 , init : 0xffff , refin : false , refout : false , xorout : 0x0000 , check : 0x772b , residue : 0x0000 } ; assert_eq ! (CRC_16_M17 , crate :: algorithm :: CRC_16_M17) ; }
};
}
