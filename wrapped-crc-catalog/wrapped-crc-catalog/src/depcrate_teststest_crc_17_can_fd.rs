// Generated macro for test_CRC_17_CAN_FD (function)
macro_rules! Depcrate_teststest_CRC_17_CAN_FD {
() => {
// Module: crate::tests
// Provides: {"test_CRC_17_CAN_FD"}
// Dependencies: {}
# [test] fn test_CRC_17_CAN_FD () { pub const CRC_17_CAN_FD : Algorithm < u32 > = Algorithm { width : 17 , poly : 0x1685b , init : 0x00000 , refin : false , refout : false , xorout : 0x00000 , check : 0x04f03 , residue : 0x00000 } ; assert_eq ! (CRC_17_CAN_FD , crate :: algorithm :: CRC_17_CAN_FD) ; }
};
}
