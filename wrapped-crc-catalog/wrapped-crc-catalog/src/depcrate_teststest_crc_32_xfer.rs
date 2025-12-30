// Generated macro for test_CRC_32_XFER (function)
macro_rules! Depcrate_teststest_CRC_32_XFER {
() => {
// Module: crate::tests
// Provides: {"test_CRC_32_XFER"}
// Dependencies: {}
# [test] fn test_CRC_32_XFER () { pub const CRC_32_XFER : Algorithm < u32 > = Algorithm { width : 32 , poly : 0x000000af , init : 0x00000000 , refin : false , refout : false , xorout : 0x00000000 , check : 0xbd0be338 , residue : 0x00000000 } ; assert_eq ! (CRC_32_XFER , crate :: algorithm :: CRC_32_XFER) ; }
};
}
