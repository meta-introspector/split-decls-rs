// Generated macro for test_CRC_16_IBM_3740 (function)
macro_rules! Depcrate_teststest_CRC_16_IBM_3740 {
() => {
// Module: crate::tests
// Provides: {"test_CRC_16_IBM_3740"}
// Dependencies: {}
# [test] fn test_CRC_16_IBM_3740 () { pub const CRC_16_IBM_3740 : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x1021 , init : 0xffff , refin : false , refout : false , xorout : 0x0000 , check : 0x29b1 , residue : 0x0000 } ; assert_eq ! (CRC_16_IBM_3740 , crate :: algorithm :: CRC_16_IBM_3740) ; }
};
}
