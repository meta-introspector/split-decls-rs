// Generated macro for test_CRC_10_GSM (function)
macro_rules! Depcrate_teststest_CRC_10_GSM {
() => {
// Module: crate::tests
// Provides: {"test_CRC_10_GSM"}
// Dependencies: {}
# [test] fn test_CRC_10_GSM () { pub const CRC_10_GSM : Algorithm < u16 > = Algorithm { width : 10 , poly : 0x175 , init : 0x000 , refin : false , refout : false , xorout : 0x3ff , check : 0x12a , residue : 0x0c6 } ; assert_eq ! (CRC_10_GSM , crate :: algorithm :: CRC_10_GSM) ; }
};
}
