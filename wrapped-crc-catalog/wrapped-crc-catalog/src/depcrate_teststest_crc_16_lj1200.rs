// Generated macro for test_CRC_16_LJ1200 (function)
macro_rules! Depcrate_teststest_CRC_16_LJ1200 {
() => {
// Module: crate::tests
// Provides: {"test_CRC_16_LJ1200"}
// Dependencies: {}
# [test] fn test_CRC_16_LJ1200 () { pub const CRC_16_LJ1200 : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x6f63 , init : 0x0000 , refin : false , refout : false , xorout : 0x0000 , check : 0xbdf4 , residue : 0x0000 } ; assert_eq ! (CRC_16_LJ1200 , crate :: algorithm :: CRC_16_LJ1200) ; }
};
}
