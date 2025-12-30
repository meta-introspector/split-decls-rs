// Generated macro for test_CRC_32_AUTOSAR (function)
macro_rules! Depcrate_teststest_CRC_32_AUTOSAR {
() => {
// Module: crate::tests
// Provides: {"test_CRC_32_AUTOSAR"}
// Dependencies: {}
# [test] fn test_CRC_32_AUTOSAR () { pub const CRC_32_AUTOSAR : Algorithm < u32 > = Algorithm { width : 32 , poly : 0xf4acfb13 , init : 0xffffffff , refin : true , refout : true , xorout : 0xffffffff , check : 0x1697d06a , residue : 0x904cddbf } ; assert_eq ! (CRC_32_AUTOSAR , crate :: algorithm :: CRC_32_AUTOSAR) ; }
};
}
