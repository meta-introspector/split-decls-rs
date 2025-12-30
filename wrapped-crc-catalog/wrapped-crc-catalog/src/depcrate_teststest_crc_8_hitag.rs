// Generated macro for test_CRC_8_HITAG (function)
macro_rules! Depcrate_teststest_CRC_8_HITAG {
() => {
// Module: crate::tests
// Provides: {"test_CRC_8_HITAG"}
// Dependencies: {}
# [test] fn test_CRC_8_HITAG () { pub const CRC_8_HITAG : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x1d , init : 0xff , refin : false , refout : false , xorout : 0x00 , check : 0xb4 , residue : 0x00 } ; assert_eq ! (CRC_8_HITAG , crate :: algorithm :: CRC_8_HITAG) ; }
};
}
