// Generated macro for impl_31 (impl)
macro_rules! Depcrate_legacyimpl_31 {
() => {
// Module: crate::legacy
// Provides: {"impl_31"}
// Dependencies: {}
impl KeyIvInit for ChaCha20LegacyCore { # [inline (always)] fn new (key : & Key , iv : & LegacyNonce) -> Self { ChaChaCore :: < R20 , Legacy > :: new (key . as_ref () , iv . as_ref ()) } }
};
}
