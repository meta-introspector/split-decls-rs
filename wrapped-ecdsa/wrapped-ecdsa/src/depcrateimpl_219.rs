// Generated macro for impl_219 (impl)
macro_rules! Depcrateimpl_219 {
() => {
// Module: crate
// Provides: {"impl_219"}
// Dependencies: {}
# [cfg (feature = "digest")] impl < C > From < SignatureWithOid < C > > for SignatureBytes < C > where C : EcdsaCurve , SignatureSize < C > : ArraySize , { fn from (signature : SignatureWithOid < C >) -> SignatureBytes < C > { signature . to_bytes () } }
};
}
