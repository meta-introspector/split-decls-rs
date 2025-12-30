// Generated macro for test_CRC_16_RIELLO (function)
macro_rules! Depcrate_teststest_CRC_16_RIELLO {
() => {
// Module: crate::tests
// Provides: {"test_CRC_16_RIELLO"}
// Dependencies: {}
# [test] fn test_CRC_16_RIELLO () { pub const CRC_16_RIELLO : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x1021 , init : 0xb2aa , refin : true , refout : true , xorout : 0x0000 , check : 0x63d0 , residue : 0x0000 } ; assert_eq ! (CRC_16_RIELLO , crate :: algorithm :: CRC_16_RIELLO) ; }
};
}
