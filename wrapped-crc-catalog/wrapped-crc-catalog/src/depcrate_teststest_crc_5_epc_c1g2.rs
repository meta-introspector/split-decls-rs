// Generated macro for test_CRC_5_EPC_C1G2 (function)
macro_rules! Depcrate_teststest_CRC_5_EPC_C1G2 {
() => {
// Module: crate::tests
// Provides: {"test_CRC_5_EPC_C1G2"}
// Dependencies: {}
# [test] fn test_CRC_5_EPC_C1G2 () { pub const CRC_5_EPC_C1G2 : Algorithm < u8 > = Algorithm { width : 5 , poly : 0x09 , init : 0x09 , refin : false , refout : false , xorout : 0x00 , check : 0x00 , residue : 0x00 } ; assert_eq ! (CRC_5_EPC_C1G2 , crate :: algorithm :: CRC_5_EPC_C1G2) ; }
};
}
