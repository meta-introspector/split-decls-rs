// Generated macro for test_CRC_64_MS (function)
macro_rules! Depcrate_teststest_CRC_64_MS {
() => {
// Module: crate::tests
// Provides: {"test_CRC_64_MS"}
// Dependencies: {}
# [test] fn test_CRC_64_MS () { pub const CRC_64_MS : Algorithm < u64 > = Algorithm { width : 64 , poly : 0x259c84cba6426349 , init : 0xffffffffffffffff , refin : true , refout : true , xorout : 0x0000000000000000 , check : 0x75d4b74f024eceea , residue : 0x0000000000000000 } ; assert_eq ! (CRC_64_MS , crate :: algorithm :: CRC_64_MS) ; }
};
}
