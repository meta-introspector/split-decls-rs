// Generated macro for test_CRC_16_KERMIT (function)
macro_rules! Depcrate_teststest_CRC_16_KERMIT {
() => {
// Module: crate::tests
// Provides: {"test_CRC_16_KERMIT"}
// Dependencies: {}
# [test] fn test_CRC_16_KERMIT () { pub const CRC_16_KERMIT : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x1021 , init : 0x0000 , refin : true , refout : true , xorout : 0x0000 , check : 0x2189 , residue : 0x0000 } ; assert_eq ! (CRC_16_KERMIT , crate :: algorithm :: CRC_16_KERMIT) ; }
};
}
