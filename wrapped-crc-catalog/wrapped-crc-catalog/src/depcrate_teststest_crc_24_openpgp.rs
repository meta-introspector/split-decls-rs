// Generated macro for test_CRC_24_OPENPGP (function)
macro_rules! Depcrate_teststest_CRC_24_OPENPGP {
() => {
// Module: crate::tests
// Provides: {"test_CRC_24_OPENPGP"}
// Dependencies: {}
# [test] fn test_CRC_24_OPENPGP () { pub const CRC_24_OPENPGP : Algorithm < u32 > = Algorithm { width : 24 , poly : 0x864cfb , init : 0xb704ce , refin : false , refout : false , xorout : 0x000000 , check : 0x21cf02 , residue : 0x000000 } ; assert_eq ! (CRC_24_OPENPGP , crate :: algorithm :: CRC_24_OPENPGP) ; }
};
}
