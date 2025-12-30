// Generated macro for test_CRC_12_DECT (function)
macro_rules! Depcrate_teststest_CRC_12_DECT {
() => {
// Module: crate::tests
// Provides: {"test_CRC_12_DECT"}
// Dependencies: {}
# [test] fn test_CRC_12_DECT () { pub const CRC_12_DECT : Algorithm < u16 > = Algorithm { width : 12 , poly : 0x80f , init : 0x000 , refin : false , refout : false , xorout : 0x000 , check : 0xf5b , residue : 0x000 } ; assert_eq ! (CRC_12_DECT , crate :: algorithm :: CRC_12_DECT) ; }
};
}
