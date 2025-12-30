// Generated macro for test_CRC_16_DNP (function)
macro_rules! Depcrate_teststest_CRC_16_DNP {
() => {
// Module: crate::tests
// Provides: {"test_CRC_16_DNP"}
// Dependencies: {}
# [test] fn test_CRC_16_DNP () { pub const CRC_16_DNP : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x3d65 , init : 0x0000 , refin : true , refout : true , xorout : 0xffff , check : 0xea82 , residue : 0x66c5 } ; assert_eq ! (CRC_16_DNP , crate :: algorithm :: CRC_16_DNP) ; }
};
}
