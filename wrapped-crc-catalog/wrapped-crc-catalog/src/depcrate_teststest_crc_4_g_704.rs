// Generated macro for test_CRC_4_G_704 (function)
macro_rules! Depcrate_teststest_CRC_4_G_704 {
() => {
// Module: crate::tests
// Provides: {"test_CRC_4_G_704"}
// Dependencies: {}
# [test] fn test_CRC_4_G_704 () { pub const CRC_4_G_704 : Algorithm < u8 > = Algorithm { width : 4 , poly : 0x3 , init : 0x0 , refin : true , refout : true , xorout : 0x0 , check : 0x7 , residue : 0x0 } ; assert_eq ! (CRC_4_G_704 , crate :: algorithm :: CRC_4_G_704) ; }
};
}
