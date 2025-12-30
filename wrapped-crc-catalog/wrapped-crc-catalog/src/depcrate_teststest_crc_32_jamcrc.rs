// Generated macro for test_CRC_32_JAMCRC (function)
macro_rules! Depcrate_teststest_CRC_32_JAMCRC {
() => {
// Module: crate::tests
// Provides: {"test_CRC_32_JAMCRC"}
// Dependencies: {}
# [test] fn test_CRC_32_JAMCRC () { pub const CRC_32_JAMCRC : Algorithm < u32 > = Algorithm { width : 32 , poly : 0x04c11db7 , init : 0xffffffff , refin : true , refout : true , xorout : 0x00000000 , check : 0x340bc6d9 , residue : 0x00000000 } ; assert_eq ! (CRC_32_JAMCRC , crate :: algorithm :: CRC_32_JAMCRC) ; }
};
}
