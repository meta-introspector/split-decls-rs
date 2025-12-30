// Generated macro for test_CRC_8_TECH_3250 (function)
macro_rules! Depcrate_teststest_CRC_8_TECH_3250 {
() => {
// Module: crate::tests
// Provides: {"test_CRC_8_TECH_3250"}
// Dependencies: {}
# [test] fn test_CRC_8_TECH_3250 () { pub const CRC_8_TECH_3250 : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x1d , init : 0xff , refin : true , refout : true , xorout : 0x00 , check : 0x97 , residue : 0x00 } ; assert_eq ! (CRC_8_TECH_3250 , crate :: algorithm :: CRC_8_TECH_3250) ; }
};
}
