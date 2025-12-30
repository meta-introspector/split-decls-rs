// Generated macro for impl_64 (impl)
macro_rules! Depcrate_public_keyimpl_64 {
() => {
// Module: crate::public_key
// Provides: {"impl_64"}
// Dependencies: {}
impl From < PublicKey > for elliptic_curve :: PublicKey < BignP256 > { fn from (value : PublicKey) -> Self { elliptic_curve :: PublicKey :: < BignP256 > :: from_affine (value . point) . expect ("should be non-identity") } }
};
}
