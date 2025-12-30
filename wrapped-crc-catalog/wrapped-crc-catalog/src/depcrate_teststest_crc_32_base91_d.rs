// Generated macro for test_CRC_32_BASE91_D (function)
macro_rules! Depcrate_teststest_CRC_32_BASE91_D {
() => {
// Module: crate::tests
// Provides: {"test_CRC_32_BASE91_D"}
// Dependencies: {}
# [test] fn test_CRC_32_BASE91_D () { pub const CRC_32_BASE91_D : Algorithm < u32 > = Algorithm { width : 32 , poly : 0xa833982b , init : 0xffffffff , refin : true , refout : true , xorout : 0xffffffff , check : 0x87315576 , residue : 0x45270551 } ; assert_eq ! (CRC_32_BASE91_D , crate :: algorithm :: CRC_32_BASE91_D) ; }
};
}
