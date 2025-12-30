// Generated macro for test_CRC_32_AIXM (function)
macro_rules! Depcrate_teststest_CRC_32_AIXM {
() => {
// Module: crate::tests
// Provides: {"test_CRC_32_AIXM"}
// Dependencies: {}
# [test] fn test_CRC_32_AIXM () { pub const CRC_32_AIXM : Algorithm < u32 > = Algorithm { width : 32 , poly : 0x814141ab , init : 0x00000000 , refin : false , refout : false , xorout : 0x00000000 , check : 0x3010bf7f , residue : 0x00000000 } ; assert_eq ! (CRC_32_AIXM , crate :: algorithm :: CRC_32_AIXM) ; }
};
}
