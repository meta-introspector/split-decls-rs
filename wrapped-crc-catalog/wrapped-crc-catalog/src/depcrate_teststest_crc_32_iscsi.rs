// Generated macro for test_CRC_32_ISCSI (function)
macro_rules! Depcrate_teststest_CRC_32_ISCSI {
() => {
// Module: crate::tests
// Provides: {"test_CRC_32_ISCSI"}
// Dependencies: {}
# [test] fn test_CRC_32_ISCSI () { pub const CRC_32_ISCSI : Algorithm < u32 > = Algorithm { width : 32 , poly : 0x1edc6f41 , init : 0xffffffff , refin : true , refout : true , xorout : 0xffffffff , check : 0xe3069283 , residue : 0xb798b438 } ; assert_eq ! (CRC_32_ISCSI , crate :: algorithm :: CRC_32_ISCSI) ; }
};
}
