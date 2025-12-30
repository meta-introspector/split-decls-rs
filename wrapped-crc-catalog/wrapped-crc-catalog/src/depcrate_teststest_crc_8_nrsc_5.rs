// Generated macro for test_CRC_8_NRSC_5 (function)
macro_rules! Depcrate_teststest_CRC_8_NRSC_5 {
() => {
// Module: crate::tests
// Provides: {"test_CRC_8_NRSC_5"}
// Dependencies: {}
# [test] fn test_CRC_8_NRSC_5 () { pub const CRC_8_NRSC_5 : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x31 , init : 0xff , refin : false , refout : false , xorout : 0x00 , check : 0xf7 , residue : 0x00 } ; assert_eq ! (CRC_8_NRSC_5 , crate :: algorithm :: CRC_8_NRSC_5) ; }
};
}
