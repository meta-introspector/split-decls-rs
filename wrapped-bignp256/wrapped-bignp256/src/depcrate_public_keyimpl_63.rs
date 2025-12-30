// Generated macro for impl_63 (impl)
macro_rules! Depcrate_public_keyimpl_63 {
() => {
// Module: crate::public_key
// Provides: {"impl_63"}
// Dependencies: {}
impl From < & PublicKey > for NonIdentity < AffinePoint > { fn from (value : & PublicKey) -> Self { PublicKey :: to_nonidentity (value) } }
};
}
