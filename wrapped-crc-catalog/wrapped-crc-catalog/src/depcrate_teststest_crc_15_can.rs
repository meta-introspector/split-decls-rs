// Generated macro for test_CRC_15_CAN (function)
macro_rules! Depcrate_teststest_CRC_15_CAN {
() => {
// Module: crate::tests
// Provides: {"test_CRC_15_CAN"}
// Dependencies: {}
# [test] fn test_CRC_15_CAN () { pub const CRC_15_CAN : Algorithm < u16 > = Algorithm { width : 15 , poly : 0x4599 , init : 0x0000 , refin : false , refout : false , xorout : 0x0000 , check : 0x059e , residue : 0x0000 } ; assert_eq ! (CRC_15_CAN , crate :: algorithm :: CRC_15_CAN) ; }
};
}
