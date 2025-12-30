// Generated macro for test_CRC_16_NRSC_5 (function)
macro_rules! Depcrate_teststest_CRC_16_NRSC_5 {
() => {
// Module: crate::tests
// Provides: {"test_CRC_16_NRSC_5"}
// Dependencies: {}
# [test] fn test_CRC_16_NRSC_5 () { pub const CRC_16_NRSC_5 : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x080b , init : 0xffff , refin : true , refout : true , xorout : 0x0000 , check : 0xa066 , residue : 0x0000 } ; assert_eq ! (CRC_16_NRSC_5 , crate :: algorithm :: CRC_16_NRSC_5) ; }
};
}
