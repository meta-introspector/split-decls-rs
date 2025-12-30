// Generated macro for test_CRC_16_USB (function)
macro_rules! Depcrate_teststest_CRC_16_USB {
() => {
// Module: crate::tests
// Provides: {"test_CRC_16_USB"}
// Dependencies: {}
# [test] fn test_CRC_16_USB () { pub const CRC_16_USB : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x8005 , init : 0xffff , refin : true , refout : true , xorout : 0xffff , check : 0xb4c8 , residue : 0xb001 } ; assert_eq ! (CRC_16_USB , crate :: algorithm :: CRC_16_USB) ; }
};
}
