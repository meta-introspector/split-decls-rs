// Generated macro for test_CRC_64_REDIS (function)
macro_rules! Depcrate_teststest_CRC_64_REDIS {
() => {
// Module: crate::tests
// Provides: {"test_CRC_64_REDIS"}
// Dependencies: {}
# [test] fn test_CRC_64_REDIS () { pub const CRC_64_REDIS : Algorithm < u64 > = Algorithm { width : 64 , poly : 0xad93d23594c935a9 , init : 0x0000000000000000 , refin : true , refout : true , xorout : 0x0000000000000000 , check : 0xe9c6d914c4b8d9ca , residue : 0x0000000000000000 } ; assert_eq ! (CRC_64_REDIS , crate :: algorithm :: CRC_64_REDIS) ; }
};
}
