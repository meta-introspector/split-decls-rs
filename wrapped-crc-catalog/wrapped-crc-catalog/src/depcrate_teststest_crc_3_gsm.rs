// Generated macro for test_CRC_3_GSM (function)
macro_rules! Depcrate_teststest_CRC_3_GSM {
() => {
// Module: crate::tests
// Provides: {"test_CRC_3_GSM"}
// Dependencies: {}
# [test] fn test_CRC_3_GSM () { pub const CRC_3_GSM : Algorithm < u8 > = Algorithm { width : 3 , poly : 0x3 , init : 0x0 , refin : false , refout : false , xorout : 0x7 , check : 0x4 , residue : 0x2 } ; assert_eq ! (CRC_3_GSM , crate :: algorithm :: CRC_3_GSM) ; }
};
}
