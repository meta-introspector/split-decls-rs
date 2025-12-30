// Generated macro for impl_206 (impl)
macro_rules! Depcrate_revocationimpl_206 {
() => {
// Module: crate::revocation
// Provides: {"impl_206"}
// Dependencies: {}
# [cfg (feature = "std")] impl TryFrom < std :: vec :: Vec < RevocationInfoChoice > > for RevocationInfoChoices { type Error = der :: Error ; fn try_from (vec : std :: vec :: Vec < RevocationInfoChoice >) -> der :: Result < RevocationInfoChoices > { Ok (RevocationInfoChoices (SetOfVec :: try_from (vec) ?)) } }
};
}
