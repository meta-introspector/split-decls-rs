// Generated macro for test_CRC_32_CD_ROM_EDC (function)
macro_rules! Depcrate_teststest_CRC_32_CD_ROM_EDC {
() => {
// Module: crate::tests
// Provides: {"test_CRC_32_CD_ROM_EDC"}
// Dependencies: {}
# [test] fn test_CRC_32_CD_ROM_EDC () { pub const CRC_32_CD_ROM_EDC : Algorithm < u32 > = Algorithm { width : 32 , poly : 0x8001801b , init : 0x00000000 , refin : true , refout : true , xorout : 0x00000000 , check : 0x6ec2edc4 , residue : 0x00000000 } ; assert_eq ! (CRC_32_CD_ROM_EDC , crate :: algorithm :: CRC_32_CD_ROM_EDC) ; }
};
}
