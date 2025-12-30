// Generated macro for test_CRC_8_OPENSAFETY (function)
macro_rules! Depcrate_teststest_CRC_8_OPENSAFETY {
() => {
// Module: crate::tests
// Provides: {"test_CRC_8_OPENSAFETY"}
// Dependencies: {}
# [test] fn test_CRC_8_OPENSAFETY () { pub const CRC_8_OPENSAFETY : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x2f , init : 0x00 , refin : false , refout : false , xorout : 0x00 , check : 0x3e , residue : 0x00 } ; assert_eq ! (CRC_8_OPENSAFETY , crate :: algorithm :: CRC_8_OPENSAFETY) ; }
};
}
