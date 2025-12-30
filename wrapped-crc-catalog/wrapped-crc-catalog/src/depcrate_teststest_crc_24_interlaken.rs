// Generated macro for test_CRC_24_INTERLAKEN (function)
macro_rules! Depcrate_teststest_CRC_24_INTERLAKEN {
() => {
// Module: crate::tests
// Provides: {"test_CRC_24_INTERLAKEN"}
// Dependencies: {}
# [test] fn test_CRC_24_INTERLAKEN () { pub const CRC_24_INTERLAKEN : Algorithm < u32 > = Algorithm { width : 24 , poly : 0x328b63 , init : 0xffffff , refin : false , refout : false , xorout : 0xffffff , check : 0xb4f3e6 , residue : 0x144e63 } ; assert_eq ! (CRC_24_INTERLAKEN , crate :: algorithm :: CRC_24_INTERLAKEN) ; }
};
}
