// Generated macro for test_CRC_5_USB (function)
macro_rules! Depcrate_teststest_CRC_5_USB {
() => {
// Module: crate::tests
// Provides: {"test_CRC_5_USB"}
// Dependencies: {}
# [test] fn test_CRC_5_USB () { pub const CRC_5_USB : Algorithm < u8 > = Algorithm { width : 5 , poly : 0x05 , init : 0x1f , refin : true , refout : true , xorout : 0x1f , check : 0x19 , residue : 0x06 } ; assert_eq ! (CRC_5_USB , crate :: algorithm :: CRC_5_USB) ; }
};
}
