// Generated macro for test_CRC_8_ROHC (function)
macro_rules! Depcrate_teststest_CRC_8_ROHC {
() => {
// Module: crate::tests
// Provides: {"test_CRC_8_ROHC"}
// Dependencies: {}
# [test] fn test_CRC_8_ROHC () { pub const CRC_8_ROHC : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x07 , init : 0xff , refin : true , refout : true , xorout : 0x00 , check : 0xd0 , residue : 0x00 } ; assert_eq ! (CRC_8_ROHC , crate :: algorithm :: CRC_8_ROHC) ; }
};
}
