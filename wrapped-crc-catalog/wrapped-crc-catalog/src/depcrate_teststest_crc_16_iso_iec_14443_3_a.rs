// Generated macro for test_CRC_16_ISO_IEC_14443_3_A (function)
macro_rules! Depcrate_teststest_CRC_16_ISO_IEC_14443_3_A {
() => {
// Module: crate::tests
// Provides: {"test_CRC_16_ISO_IEC_14443_3_A"}
// Dependencies: {}
# [test] fn test_CRC_16_ISO_IEC_14443_3_A () { pub const CRC_16_ISO_IEC_14443_3_A : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x1021 , init : 0xc6c6 , refin : true , refout : true , xorout : 0x0000 , check : 0xbf05 , residue : 0x0000 } ; assert_eq ! (CRC_16_ISO_IEC_14443_3_A , crate :: algorithm :: CRC_16_ISO_IEC_14443_3_A) ; }
};
}
