// Generated macro for test_CRC_64_ECMA_182 (function)
macro_rules! Depcrate_teststest_CRC_64_ECMA_182 {
() => {
// Module: crate::tests
// Provides: {"test_CRC_64_ECMA_182"}
// Dependencies: {}
# [test] fn test_CRC_64_ECMA_182 () { pub const CRC_64_ECMA_182 : Algorithm < u64 > = Algorithm { width : 64 , poly : 0x42f0e1eba9ea3693 , init : 0x0000000000000000 , refin : false , refout : false , xorout : 0x0000000000000000 , check : 0x6c40df5f0b497347 , residue : 0x0000000000000000 } ; assert_eq ! (CRC_64_ECMA_182 , crate :: algorithm :: CRC_64_ECMA_182) ; }
};
}
