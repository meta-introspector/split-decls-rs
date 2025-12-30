// Generated macro for test_CRC_13_BBC (function)
macro_rules! Depcrate_teststest_CRC_13_BBC {
() => {
// Module: crate::tests
// Provides: {"test_CRC_13_BBC"}
// Dependencies: {}
# [test] fn test_CRC_13_BBC () { pub const CRC_13_BBC : Algorithm < u16 > = Algorithm { width : 13 , poly : 0x1cf5 , init : 0x0000 , refin : false , refout : false , xorout : 0x0000 , check : 0x04fa , residue : 0x0000 } ; assert_eq ! (CRC_13_BBC , crate :: algorithm :: CRC_13_BBC) ; }
};
}
