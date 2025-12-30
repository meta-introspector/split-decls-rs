// Generated macro for test_CRC_16_DDS_110 (function)
macro_rules! Depcrate_teststest_CRC_16_DDS_110 {
() => {
// Module: crate::tests
// Provides: {"test_CRC_16_DDS_110"}
// Dependencies: {}
# [test] fn test_CRC_16_DDS_110 () { pub const CRC_16_DDS_110 : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x8005 , init : 0x800d , refin : false , refout : false , xorout : 0x0000 , check : 0x9ecf , residue : 0x0000 } ; assert_eq ! (CRC_16_DDS_110 , crate :: algorithm :: CRC_16_DDS_110) ; }
};
}
