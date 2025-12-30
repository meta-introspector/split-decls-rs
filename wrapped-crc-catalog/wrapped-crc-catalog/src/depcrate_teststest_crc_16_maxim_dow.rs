// Generated macro for test_CRC_16_MAXIM_DOW (function)
macro_rules! Depcrate_teststest_CRC_16_MAXIM_DOW {
() => {
// Module: crate::tests
// Provides: {"test_CRC_16_MAXIM_DOW"}
// Dependencies: {}
# [test] fn test_CRC_16_MAXIM_DOW () { pub const CRC_16_MAXIM_DOW : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x8005 , init : 0x0000 , refin : true , refout : true , xorout : 0xffff , check : 0x44c2 , residue : 0xb001 } ; assert_eq ! (CRC_16_MAXIM_DOW , crate :: algorithm :: CRC_16_MAXIM_DOW) ; }
};
}
