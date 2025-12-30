// Generated macro for impl_114 (impl)
macro_rules! Depcrateimpl_114 {
() => {
// Module: crate
// Provides: {"impl_114"}
// Dependencies: {}
impl GroupDigest for Ed448 { const HASH_TO_CURVE_ID : & [u8] = b"edwards448_XOF:SHAKE256_ELL2_RO_" ; const ENCODE_TO_CURVE_ID : & [u8] = b"edwards448_XOF:SHAKE256_ELL2_NU_" ; type ExpandMsg = ExpandMsgXof < Shake256 > ; }
};
}
