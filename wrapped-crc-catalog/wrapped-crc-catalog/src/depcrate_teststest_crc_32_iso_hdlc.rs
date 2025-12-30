// Generated macro for test_CRC_32_ISO_HDLC (function)
macro_rules! Depcrate_teststest_CRC_32_ISO_HDLC {
() => {
// Module: crate::tests
// Provides: {"test_CRC_32_ISO_HDLC"}
// Dependencies: {}
# [test] fn test_CRC_32_ISO_HDLC () { pub const CRC_32_ISO_HDLC : Algorithm < u32 > = Algorithm { width : 32 , poly : 0x04c11db7 , init : 0xffffffff , refin : true , refout : true , xorout : 0xffffffff , check : 0xcbf43926 , residue : 0xdebb20e3 } ; assert_eq ! (CRC_32_ISO_HDLC , crate :: algorithm :: CRC_32_ISO_HDLC) ; }
};
}
