// Generated macro for test_CRC_16_TMS37157 (function)
macro_rules! Depcrate_teststest_CRC_16_TMS37157 {
() => {
// Module: crate::tests
// Provides: {"test_CRC_16_TMS37157"}
// Dependencies: {}
# [test] fn test_CRC_16_TMS37157 () { pub const CRC_16_TMS37157 : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x1021 , init : 0x89ec , refin : true , refout : true , xorout : 0x0000 , check : 0x26b1 , residue : 0x0000 } ; assert_eq ! (CRC_16_TMS37157 , crate :: algorithm :: CRC_16_TMS37157) ; }
};
}
