// Generated macro for impl_218 (impl)
macro_rules! Depcrateimpl_218 {
() => {
// Module: crate
// Provides: {"impl_218"}
// Dependencies: {}
# [cfg (feature = "digest")] impl < C > From < SignatureWithOid < C > > for Signature < C > where C : EcdsaCurve , { fn from (sig : SignatureWithOid < C >) -> Signature < C > { sig . signature } }
};
}
