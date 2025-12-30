// Generated macro for test_CRC_30_CDMA (function)
macro_rules! Depcrate_teststest_CRC_30_CDMA {
() => {
// Module: crate::tests
// Provides: {"test_CRC_30_CDMA"}
// Dependencies: {}
# [test] fn test_CRC_30_CDMA () { pub const CRC_30_CDMA : Algorithm < u32 > = Algorithm { width : 30 , poly : 0x2030b9c7 , init : 0x3fffffff , refin : false , refout : false , xorout : 0x3fffffff , check : 0x04c34abf , residue : 0x34efa55a } ; assert_eq ! (CRC_30_CDMA , crate :: algorithm :: CRC_30_CDMA) ; }
};
}
