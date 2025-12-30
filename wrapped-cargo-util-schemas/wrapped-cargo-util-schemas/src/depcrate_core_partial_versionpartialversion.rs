// Generated macro for PartialVersion (struct)
macro_rules! Depcrate_core_partial_versionPartialVersion {
() => {
// Module: crate::core::partial_version
// Provides: {"PartialVersion"}
// Dependencies: {}
# [derive (PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Debug)] pub struct PartialVersion { pub major : u64 , pub minor : Option < u64 > , pub patch : Option < u64 > , pub pre : Option < semver :: Prerelease > , pub build : Option < semver :: BuildMetadata > , }
};
}
