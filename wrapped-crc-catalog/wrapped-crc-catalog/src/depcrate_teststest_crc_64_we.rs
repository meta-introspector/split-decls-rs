// Generated macro for test_CRC_64_WE (function)
macro_rules! Depcrate_teststest_CRC_64_WE {
() => {
// Module: crate::tests
// Provides: {"test_CRC_64_WE"}
// Dependencies: {}
# [test] fn test_CRC_64_WE () { pub const CRC_64_WE : Algorithm < u64 > = Algorithm { width : 64 , poly : 0x42f0e1eba9ea3693 , init : 0xffffffffffffffff , refin : false , refout : false , xorout : 0xffffffffffffffff , check : 0x62ec59e3f1a4f00a , residue : 0xfcacbebd5931a992 } ; assert_eq ! (CRC_64_WE , crate :: algorithm :: CRC_64_WE) ; }
};
}
