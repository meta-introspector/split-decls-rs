// Generated macro for test_CRC_16_IBM_SDLC (function)
macro_rules! Depcrate_teststest_CRC_16_IBM_SDLC {
() => {
// Module: crate::tests
// Provides: {"test_CRC_16_IBM_SDLC"}
// Dependencies: {}
# [test] fn test_CRC_16_IBM_SDLC () { pub const CRC_16_IBM_SDLC : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x1021 , init : 0xffff , refin : true , refout : true , xorout : 0xffff , check : 0x906e , residue : 0xf0b8 } ; assert_eq ! (CRC_16_IBM_SDLC , crate :: algorithm :: CRC_16_IBM_SDLC) ; }
};
}
