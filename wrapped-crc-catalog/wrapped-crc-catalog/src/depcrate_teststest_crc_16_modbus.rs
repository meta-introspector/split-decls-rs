// Generated macro for test_CRC_16_MODBUS (function)
macro_rules! Depcrate_teststest_CRC_16_MODBUS {
() => {
// Module: crate::tests
// Provides: {"test_CRC_16_MODBUS"}
// Dependencies: {}
# [test] fn test_CRC_16_MODBUS () { pub const CRC_16_MODBUS : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x8005 , init : 0xffff , refin : true , refout : true , xorout : 0x0000 , check : 0x4b37 , residue : 0x0000 } ; assert_eq ! (CRC_16_MODBUS , crate :: algorithm :: CRC_16_MODBUS) ; }
};
}
