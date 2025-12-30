// Generated macro for test_CRC_15_MPT1327 (function)
macro_rules! Depcrate_teststest_CRC_15_MPT1327 {
() => {
// Module: crate::tests
// Provides: {"test_CRC_15_MPT1327"}
// Dependencies: {}
# [test] fn test_CRC_15_MPT1327 () { pub const CRC_15_MPT1327 : Algorithm < u16 > = Algorithm { width : 15 , poly : 0x6815 , init : 0x0000 , refin : false , refout : false , xorout : 0x0001 , check : 0x2566 , residue : 0x6815 } ; assert_eq ! (CRC_15_MPT1327 , crate :: algorithm :: CRC_15_MPT1327) ; }
};
}
