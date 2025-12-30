// Generated macro for impl_168 (impl)
macro_rules! Depcrate_enveloped_dataimpl_168 {
() => {
// Module: crate::enveloped_data
// Provides: {"impl_168"}
// Dependencies: {}
# [cfg (feature = "std")] impl TryFrom < std :: vec :: Vec < RecipientInfo > > for RecipientInfos { type Error = der :: Error ; fn try_from (vec : std :: vec :: Vec < RecipientInfo >) -> der :: Result < RecipientInfos > { Ok (RecipientInfos (SetOfVec :: try_from (vec) ?)) } }
};
}
