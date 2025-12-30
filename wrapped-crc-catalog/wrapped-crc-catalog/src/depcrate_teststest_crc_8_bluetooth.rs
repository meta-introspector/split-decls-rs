// Generated macro for test_CRC_8_BLUETOOTH (function)
macro_rules! Depcrate_teststest_CRC_8_BLUETOOTH {
() => {
// Module: crate::tests
// Provides: {"test_CRC_8_BLUETOOTH"}
// Dependencies: {}
# [test] fn test_CRC_8_BLUETOOTH () { pub const CRC_8_BLUETOOTH : Algorithm < u8 > = Algorithm { width : 8 , poly : 0xa7 , init : 0x00 , refin : true , refout : true , xorout : 0x00 , check : 0x26 , residue : 0x00 } ; assert_eq ! (CRC_8_BLUETOOTH , crate :: algorithm :: CRC_8_BLUETOOTH) ; }
};
}
