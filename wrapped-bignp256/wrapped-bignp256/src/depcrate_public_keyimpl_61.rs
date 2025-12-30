// Generated macro for impl_61 (impl)
macro_rules! Depcrate_public_keyimpl_61 {
() => {
// Module: crate::public_key
// Provides: {"impl_61"}
// Dependencies: {}
impl From < & NonIdentity < AffinePoint > > for PublicKey { fn from (value : & NonIdentity < AffinePoint >) -> Self { Self { point : value . to_point () , } } }
};
}
