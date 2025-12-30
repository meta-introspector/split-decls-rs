// Generated macro for impl_42 (impl)
macro_rules! Depcrate_ecdsaimpl_42 {
() => {
// Module: crate::ecdsa
// Provides: {"impl_42"}
// Dependencies: {}
# [cfg (feature = "arithmetic")] impl Signature { # [doc = " Get the `s0` word component of this signature"] pub fn s0 (& self) -> NonZeroScalar { let mut s0 = self . s0 . to_bytes () ; s0 . reverse () ; NonZeroScalar :: new (Scalar :: from_bytes (& s0) . unwrap ()) . unwrap () } # [doc = " Get the `s1` word component of this signature"] pub fn s1 (& self) -> NonZeroScalar { let mut s1 = self . s1 . to_bytes () ; s1 . reverse () ; NonZeroScalar :: new (Scalar :: from_bytes (& s1) . unwrap ()) . unwrap () } # [doc = " Split the signature into its `s0` and `s1` scalars."] pub fn split_scalars (& self) -> (NonZeroScalar , NonZeroScalar) { (self . s0 () , self . s1 ()) } }
};
}
