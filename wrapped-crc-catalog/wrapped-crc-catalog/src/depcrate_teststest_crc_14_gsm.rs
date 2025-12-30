// Generated macro for test_CRC_14_GSM (function)
macro_rules! Depcrate_teststest_CRC_14_GSM {
() => {
// Module: crate::tests
// Provides: {"test_CRC_14_GSM"}
// Dependencies: {}
# [test] fn test_CRC_14_GSM () { pub const CRC_14_GSM : Algorithm < u16 > = Algorithm { width : 14 , poly : 0x202d , init : 0x0000 , refin : false , refout : false , xorout : 0x3fff , check : 0x30ae , residue : 0x031e } ; assert_eq ! (CRC_14_GSM , crate :: algorithm :: CRC_14_GSM) ; }
};
}
