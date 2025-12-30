// Generated macro for test_CRC_32_MPEG_2 (function)
macro_rules! Depcrate_teststest_CRC_32_MPEG_2 {
() => {
// Module: crate::tests
// Provides: {"test_CRC_32_MPEG_2"}
// Dependencies: {}
# [test] fn test_CRC_32_MPEG_2 () { pub const CRC_32_MPEG_2 : Algorithm < u32 > = Algorithm { width : 32 , poly : 0x04c11db7 , init : 0xffffffff , refin : false , refout : false , xorout : 0x00000000 , check : 0x0376e6e7 , residue : 0x00000000 } ; assert_eq ! (CRC_32_MPEG_2 , crate :: algorithm :: CRC_32_MPEG_2) ; }
};
}
