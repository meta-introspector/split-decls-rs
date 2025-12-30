// Generated macro for test_CRC_64_GO_ISO (function)
macro_rules! Depcrate_teststest_CRC_64_GO_ISO {
() => {
// Module: crate::tests
// Provides: {"test_CRC_64_GO_ISO"}
// Dependencies: {}
# [test] fn test_CRC_64_GO_ISO () { pub const CRC_64_GO_ISO : Algorithm < u64 > = Algorithm { width : 64 , poly : 0x000000000000001b , init : 0xffffffffffffffff , refin : true , refout : true , xorout : 0xffffffffffffffff , check : 0xb90956c775a41001 , residue : 0x5300000000000000 } ; assert_eq ! (CRC_64_GO_ISO , crate :: algorithm :: CRC_64_GO_ISO) ; }
};
}
