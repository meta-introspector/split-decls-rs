// Generated macro for test_CRC_8_MAXIM_DOW (function)
macro_rules! Depcrate_teststest_CRC_8_MAXIM_DOW {
() => {
// Module: crate::tests
// Provides: {"test_CRC_8_MAXIM_DOW"}
// Dependencies: {}
# [test] fn test_CRC_8_MAXIM_DOW () { pub const CRC_8_MAXIM_DOW : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x31 , init : 0x00 , refin : true , refout : true , xorout : 0x00 , check : 0xa1 , residue : 0x00 } ; assert_eq ! (CRC_8_MAXIM_DOW , crate :: algorithm :: CRC_8_MAXIM_DOW) ; }
};
}
