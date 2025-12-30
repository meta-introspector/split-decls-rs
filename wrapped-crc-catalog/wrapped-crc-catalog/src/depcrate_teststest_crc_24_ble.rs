// Generated macro for test_CRC_24_BLE (function)
macro_rules! Depcrate_teststest_CRC_24_BLE {
() => {
// Module: crate::tests
// Provides: {"test_CRC_24_BLE"}
// Dependencies: {}
# [test] fn test_CRC_24_BLE () { pub const CRC_24_BLE : Algorithm < u32 > = Algorithm { width : 24 , poly : 0x00065b , init : 0x555555 , refin : true , refout : true , xorout : 0x000000 , check : 0xc25a56 , residue : 0x000000 } ; assert_eq ! (CRC_24_BLE , crate :: algorithm :: CRC_24_BLE) ; }
};
}
