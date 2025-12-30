// Generated macro for test_CRC_16_T10_DIF (function)
macro_rules! Depcrate_teststest_CRC_16_T10_DIF {
() => {
// Module: crate::tests
// Provides: {"test_CRC_16_T10_DIF"}
// Dependencies: {}
# [test] fn test_CRC_16_T10_DIF () { pub const CRC_16_T10_DIF : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x8bb7 , init : 0x0000 , refin : false , refout : false , xorout : 0x0000 , check : 0xd0db , residue : 0x0000 } ; assert_eq ! (CRC_16_T10_DIF , crate :: algorithm :: CRC_16_T10_DIF) ; }
};
}
