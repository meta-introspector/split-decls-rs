// Generated macro for test_CRC_7_ROHC (function)
macro_rules! Depcrate_teststest_CRC_7_ROHC {
() => {
// Module: crate::tests
// Provides: {"test_CRC_7_ROHC"}
// Dependencies: {}
# [test] fn test_CRC_7_ROHC () { pub const CRC_7_ROHC : Algorithm < u8 > = Algorithm { width : 7 , poly : 0x4f , init : 0x7f , refin : true , refout : true , xorout : 0x00 , check : 0x53 , residue : 0x00 } ; assert_eq ! (CRC_7_ROHC , crate :: algorithm :: CRC_7_ROHC) ; }
};
}
