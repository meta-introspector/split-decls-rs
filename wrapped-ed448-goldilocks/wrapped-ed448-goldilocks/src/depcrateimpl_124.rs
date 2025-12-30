// Generated macro for impl_124 (impl)
macro_rules! Depcrateimpl_124 {
() => {
// Module: crate
// Provides: {"impl_124"}
// Dependencies: {}
impl GroupDigest for Decaf448 { const HASH_TO_CURVE_ID : & [u8] = b"decaf448_XOF:SHAKE256_D448MAP_RO_" ; const ENCODE_TO_CURVE_ID : & [u8] = b"decaf448_XOF:SHAKE256_D448MAP_NU_" ; type ExpandMsg = ExpandMsgXof < Shake256 > ; }
};
}
