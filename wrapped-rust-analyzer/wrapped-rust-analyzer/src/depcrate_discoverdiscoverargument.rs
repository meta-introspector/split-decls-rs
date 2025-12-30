// Generated macro for DiscoverArgument (enum)
macro_rules! Depcrate_discoverDiscoverArgument {
() => {
// Module: crate::discover
// Provides: {"DiscoverArgument"}
// Dependencies: {}
# [derive (PartialEq , Clone , Debug , Serialize)] # [serde (rename_all = "camelCase")] pub (crate) enum DiscoverArgument { Path (# [serde (serialize_with = "serialize_abs_pathbuf")] AbsPathBuf) , Buildfile (# [serde (serialize_with = "serialize_abs_pathbuf")] AbsPathBuf) , }
};
}
