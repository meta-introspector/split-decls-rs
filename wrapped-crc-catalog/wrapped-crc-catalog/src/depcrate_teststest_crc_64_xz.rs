// Generated macro for test_CRC_64_XZ (function)
macro_rules! Depcrate_teststest_CRC_64_XZ {
() => {
// Module: crate::tests
// Provides: {"test_CRC_64_XZ"}
// Dependencies: {}
# [test] fn test_CRC_64_XZ () { pub const CRC_64_XZ : Algorithm < u64 > = Algorithm { width : 64 , poly : 0x42f0e1eba9ea3693 , init : 0xffffffffffffffff , refin : true , refout : true , xorout : 0xffffffffffffffff , check : 0x995dc9bbdf1939fa , residue : 0x49958c9abd7d353f } ; assert_eq ! (CRC_64_XZ , crate :: algorithm :: CRC_64_XZ) ; }
};
}
