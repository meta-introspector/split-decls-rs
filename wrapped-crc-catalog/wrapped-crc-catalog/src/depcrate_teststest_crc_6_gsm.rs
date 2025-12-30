// Generated macro for test_CRC_6_GSM (function)
macro_rules! Depcrate_teststest_CRC_6_GSM {
() => {
// Module: crate::tests
// Provides: {"test_CRC_6_GSM"}
// Dependencies: {}
# [test] fn test_CRC_6_GSM () { pub const CRC_6_GSM : Algorithm < u8 > = Algorithm { width : 6 , poly : 0x2f , init : 0x00 , refin : false , refout : false , xorout : 0x3f , check : 0x13 , residue : 0x3a } ; assert_eq ! (CRC_6_GSM , crate :: algorithm :: CRC_6_GSM) ; }
};
}
