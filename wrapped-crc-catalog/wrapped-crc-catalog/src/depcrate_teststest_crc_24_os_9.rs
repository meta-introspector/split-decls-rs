// Generated macro for test_CRC_24_OS_9 (function)
macro_rules! Depcrate_teststest_CRC_24_OS_9 {
() => {
// Module: crate::tests
// Provides: {"test_CRC_24_OS_9"}
// Dependencies: {}
# [test] fn test_CRC_24_OS_9 () { pub const CRC_24_OS_9 : Algorithm < u32 > = Algorithm { width : 24 , poly : 0x800063 , init : 0xffffff , refin : false , refout : false , xorout : 0xffffff , check : 0x200fa5 , residue : 0x800fe3 } ; assert_eq ! (CRC_24_OS_9 , crate :: algorithm :: CRC_24_OS_9) ; }
};
}
