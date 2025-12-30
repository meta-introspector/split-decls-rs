// Generated macro for impl_224 (impl)
macro_rules! Depcrate_signed_dataimpl_224 {
() => {
// Module: crate::signed_data
// Provides: {"impl_224"}
// Dependencies: {}
# [cfg (feature = "std")] impl TryFrom < std :: vec :: Vec < SignerInfo > > for SignerInfos { type Error = der :: Error ; fn try_from (vec : std :: vec :: Vec < SignerInfo >) -> der :: Result < SignerInfos > { Ok (SignerInfos (SetOfVec :: try_from (vec) ?)) } }
};
}
