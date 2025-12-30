// Generated macro for test_CRC_7_MMC (function)
macro_rules! Depcrate_teststest_CRC_7_MMC {
() => {
// Module: crate::tests
// Provides: {"test_CRC_7_MMC"}
// Dependencies: {}
# [test] fn test_CRC_7_MMC () { pub const CRC_7_MMC : Algorithm < u8 > = Algorithm { width : 7 , poly : 0x09 , init : 0x00 , refin : false , refout : false , xorout : 0x00 , check : 0x75 , residue : 0x00 } ; assert_eq ! (CRC_7_MMC , crate :: algorithm :: CRC_7_MMC) ; }
};
}
