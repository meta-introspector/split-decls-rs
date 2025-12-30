// Generated macro for test_CRC_8_AUTOSAR (function)
macro_rules! Depcrate_teststest_CRC_8_AUTOSAR {
() => {
// Module: crate::tests
// Provides: {"test_CRC_8_AUTOSAR"}
// Dependencies: {}
# [test] fn test_CRC_8_AUTOSAR () { pub const CRC_8_AUTOSAR : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x2f , init : 0xff , refin : false , refout : false , xorout : 0xff , check : 0xdf , residue : 0x42 } ; assert_eq ! (CRC_8_AUTOSAR , crate :: algorithm :: CRC_8_AUTOSAR) ; }
};
}
