// Generated macro for test_CRC_12_GSM (function)
macro_rules! Depcrate_teststest_CRC_12_GSM {
() => {
// Module: crate::tests
// Provides: {"test_CRC_12_GSM"}
// Dependencies: {}
# [test] fn test_CRC_12_GSM () { pub const CRC_12_GSM : Algorithm < u16 > = Algorithm { width : 12 , poly : 0xd31 , init : 0x000 , refin : false , refout : false , xorout : 0xfff , check : 0xb34 , residue : 0x178 } ; assert_eq ! (CRC_12_GSM , crate :: algorithm :: CRC_12_GSM) ; }
};
}
