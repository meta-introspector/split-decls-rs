// Generated macro for impl_221 (impl)
macro_rules! Depcrate_signed_dataimpl_221 {
() => {
// Module: crate::signed_data
// Provides: {"impl_221"}
// Dependencies: {}
# [cfg (feature = "std")] impl TryFrom < std :: vec :: Vec < CertificateChoices > > for CertificateSet { type Error = der :: Error ; fn try_from (vec : std :: vec :: Vec < CertificateChoices >) -> der :: Result < CertificateSet > { Ok (CertificateSet (SetOfVec :: try_from (vec) ?)) } }
};
}
