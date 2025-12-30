// Generated macro for test_CRC_32_BZIP2 (function)
macro_rules! Depcrate_teststest_CRC_32_BZIP2 {
() => {
// Module: crate::tests
// Provides: {"test_CRC_32_BZIP2"}
// Dependencies: {}
# [test] fn test_CRC_32_BZIP2 () { pub const CRC_32_BZIP2 : Algorithm < u32 > = Algorithm { width : 32 , poly : 0x04c11db7 , init : 0xffffffff , refin : false , refout : false , xorout : 0xffffffff , check : 0xfc891918 , residue : 0xc704dd7b } ; assert_eq ! (CRC_32_BZIP2 , crate :: algorithm :: CRC_32_BZIP2) ; }
};
}
