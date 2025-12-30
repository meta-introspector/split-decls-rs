// Generated macro for test_CRC_16_SPI_FUJITSU (function)
macro_rules! Depcrate_teststest_CRC_16_SPI_FUJITSU {
() => {
// Module: crate::tests
// Provides: {"test_CRC_16_SPI_FUJITSU"}
// Dependencies: {}
# [test] fn test_CRC_16_SPI_FUJITSU () { pub const CRC_16_SPI_FUJITSU : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x1021 , init : 0x1d0f , refin : false , refout : false , xorout : 0x0000 , check : 0xe5cc , residue : 0x0000 } ; assert_eq ! (CRC_16_SPI_FUJITSU , crate :: algorithm :: CRC_16_SPI_FUJITSU) ; }
};
}
