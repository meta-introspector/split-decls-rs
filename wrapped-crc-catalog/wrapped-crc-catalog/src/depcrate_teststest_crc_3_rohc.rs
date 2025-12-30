// Generated macro for test_CRC_3_ROHC (function)
macro_rules! Depcrate_teststest_CRC_3_ROHC {
() => {
// Module: crate::tests
// Provides: {"test_CRC_3_ROHC"}
// Dependencies: {}
# [test] fn test_CRC_3_ROHC () { pub const CRC_3_ROHC : Algorithm < u8 > = Algorithm { width : 3 , poly : 0x3 , init : 0x7 , refin : true , refout : true , xorout : 0x0 , check : 0x6 , residue : 0x0 } ; assert_eq ! (CRC_3_ROHC , crate :: algorithm :: CRC_3_ROHC) ; }
};
}
