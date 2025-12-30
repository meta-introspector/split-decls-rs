// Generated macro for test_CRC_16_MCRF4XX (function)
macro_rules! Depcrate_teststest_CRC_16_MCRF4XX {
() => {
// Module: crate::tests
// Provides: {"test_CRC_16_MCRF4XX"}
// Dependencies: {}
# [test] fn test_CRC_16_MCRF4XX () { pub const CRC_16_MCRF4XX : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x1021 , init : 0xffff , refin : true , refout : true , xorout : 0x0000 , check : 0x6f91 , residue : 0x0000 } ; assert_eq ! (CRC_16_MCRF4XX , crate :: algorithm :: CRC_16_MCRF4XX) ; }
};
}
