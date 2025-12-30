// Generated macro for test_CRC_21_CAN_FD (function)
macro_rules! Depcrate_teststest_CRC_21_CAN_FD {
() => {
// Module: crate::tests
// Provides: {"test_CRC_21_CAN_FD"}
// Dependencies: {}
# [test] fn test_CRC_21_CAN_FD () { pub const CRC_21_CAN_FD : Algorithm < u32 > = Algorithm { width : 21 , poly : 0x102899 , init : 0x000000 , refin : false , refout : false , xorout : 0x000000 , check : 0x0ed841 , residue : 0x000000 } ; assert_eq ! (CRC_21_CAN_FD , crate :: algorithm :: CRC_21_CAN_FD) ; }
};
}
