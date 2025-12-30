// Generated macro for test_CRC_32_CKSUM (function)
macro_rules! Depcrate_teststest_CRC_32_CKSUM {
() => {
// Module: crate::tests
// Provides: {"test_CRC_32_CKSUM"}
// Dependencies: {}
# [test] fn test_CRC_32_CKSUM () { pub const CRC_32_CKSUM : Algorithm < u32 > = Algorithm { width : 32 , poly : 0x04c11db7 , init : 0x00000000 , refin : false , refout : false , xorout : 0xffffffff , check : 0x765e7680 , residue : 0xc704dd7b } ; assert_eq ! (CRC_32_CKSUM , crate :: algorithm :: CRC_32_CKSUM) ; }
};
}
