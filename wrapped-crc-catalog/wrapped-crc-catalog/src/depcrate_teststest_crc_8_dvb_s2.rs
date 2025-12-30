// Generated macro for test_CRC_8_DVB_S2 (function)
macro_rules! Depcrate_teststest_CRC_8_DVB_S2 {
() => {
// Module: crate::tests
// Provides: {"test_CRC_8_DVB_S2"}
// Dependencies: {}
# [test] fn test_CRC_8_DVB_S2 () { pub const CRC_8_DVB_S2 : Algorithm < u8 > = Algorithm { width : 8 , poly : 0xd5 , init : 0x00 , refin : false , refout : false , xorout : 0x00 , check : 0xbc , residue : 0x00 } ; assert_eq ! (CRC_8_DVB_S2 , crate :: algorithm :: CRC_8_DVB_S2) ; }
};
}
