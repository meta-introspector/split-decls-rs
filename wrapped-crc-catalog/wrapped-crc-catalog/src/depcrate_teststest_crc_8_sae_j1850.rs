// Generated macro for test_CRC_8_SAE_J1850 (function)
macro_rules! Depcrate_teststest_CRC_8_SAE_J1850 {
() => {
// Module: crate::tests
// Provides: {"test_CRC_8_SAE_J1850"}
// Dependencies: {}
# [test] fn test_CRC_8_SAE_J1850 () { pub const CRC_8_SAE_J1850 : Algorithm < u8 > = Algorithm { width : 8 , poly : 0x1d , init : 0xff , refin : false , refout : false , xorout : 0xff , check : 0x4b , residue : 0xc4 } ; assert_eq ! (CRC_8_SAE_J1850 , crate :: algorithm :: CRC_8_SAE_J1850) ; }
};
}
