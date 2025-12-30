// Generated macro for test_CRC_10_ATM (function)
macro_rules! Depcrate_teststest_CRC_10_ATM {
() => {
// Module: crate::tests
// Provides: {"test_CRC_10_ATM"}
// Dependencies: {}
# [test] fn test_CRC_10_ATM () { pub const CRC_10_ATM : Algorithm < u16 > = Algorithm { width : 10 , poly : 0x233 , init : 0x000 , refin : false , refout : false , xorout : 0x000 , check : 0x199 , residue : 0x000 } ; assert_eq ! (CRC_10_ATM , crate :: algorithm :: CRC_10_ATM) ; }
};
}
