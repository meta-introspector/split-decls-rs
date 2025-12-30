// Generated macro for test_CRC_16_GSM (function)
macro_rules! Depcrate_teststest_CRC_16_GSM {
() => {
// Module: crate::tests
// Provides: {"test_CRC_16_GSM"}
// Dependencies: {}
# [test] fn test_CRC_16_GSM () { pub const CRC_16_GSM : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x1021 , init : 0x0000 , refin : false , refout : false , xorout : 0xffff , check : 0xce3c , residue : 0x1d0f } ; assert_eq ! (CRC_16_GSM , crate :: algorithm :: CRC_16_GSM) ; }
};
}
