// Generated macro for test_CRC_6_G_704 (function)
macro_rules! Depcrate_teststest_CRC_6_G_704 {
() => {
// Module: crate::tests
// Provides: {"test_CRC_6_G_704"}
// Dependencies: {}
# [test] fn test_CRC_6_G_704 () { pub const CRC_6_G_704 : Algorithm < u8 > = Algorithm { width : 6 , poly : 0x03 , init : 0x00 , refin : true , refout : true , xorout : 0x00 , check : 0x06 , residue : 0x00 } ; assert_eq ! (CRC_6_G_704 , crate :: algorithm :: CRC_6_G_704) ; }
};
}
