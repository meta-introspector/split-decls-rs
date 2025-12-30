// Generated macro for test_CRC_40_GSM (function)
macro_rules! Depcrate_teststest_CRC_40_GSM {
() => {
// Module: crate::tests
// Provides: {"test_CRC_40_GSM"}
// Dependencies: {}
# [test] fn test_CRC_40_GSM () { pub const CRC_40_GSM : Algorithm < u64 > = Algorithm { width : 40 , poly : 0x0004820009 , init : 0x0000000000 , refin : false , refout : false , xorout : 0xffffffffff , check : 0xd4164fc646 , residue : 0xc4ff8071ff } ; assert_eq ! (CRC_40_GSM , crate :: algorithm :: CRC_40_GSM) ; }
};
}
