// Generated macro for test_CRC_32_MEF (function)
macro_rules! Depcrate_teststest_CRC_32_MEF {
() => {
// Module: crate::tests
// Provides: {"test_CRC_32_MEF"}
// Dependencies: {}
# [test] fn test_CRC_32_MEF () { pub const CRC_32_MEF : Algorithm < u32 > = Algorithm { width : 32 , poly : 0x741b8cd7 , init : 0xffffffff , refin : true , refout : true , xorout : 0x00000000 , check : 0xd2c22f51 , residue : 0x00000000 } ; assert_eq ! (CRC_32_MEF , crate :: algorithm :: CRC_32_MEF) ; }
};
}
