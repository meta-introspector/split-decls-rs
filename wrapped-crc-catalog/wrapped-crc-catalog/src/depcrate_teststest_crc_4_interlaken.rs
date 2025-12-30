// Generated macro for test_CRC_4_INTERLAKEN (function)
macro_rules! Depcrate_teststest_CRC_4_INTERLAKEN {
() => {
// Module: crate::tests
// Provides: {"test_CRC_4_INTERLAKEN"}
// Dependencies: {}
# [test] fn test_CRC_4_INTERLAKEN () { pub const CRC_4_INTERLAKEN : Algorithm < u8 > = Algorithm { width : 4 , poly : 0x3 , init : 0xf , refin : false , refout : false , xorout : 0xf , check : 0xb , residue : 0x2 } ; assert_eq ! (CRC_4_INTERLAKEN , crate :: algorithm :: CRC_4_INTERLAKEN) ; }
};
}
