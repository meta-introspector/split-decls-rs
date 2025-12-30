// Generated macro for impl_202 (impl)
macro_rules! Depcrateimpl_202 {
() => {
// Module: crate
// Provides: {"impl_202"}
// Dependencies: {}
impl < C > From < Signature < C > > for SignatureBytes < C > where C : EcdsaCurve , SignatureSize < C > : ArraySize , { fn from (signature : Signature < C >) -> SignatureBytes < C > { signature . to_bytes () } }
};
}
