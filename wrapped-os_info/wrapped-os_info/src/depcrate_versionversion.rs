// Generated macro for Version (enum)
macro_rules! Depcrate_versionVersion {
() => {
// Module: crate::version
// Provides: {"Version"}
// Dependencies: {}
# [doc = " Operating system version."] # [derive (Debug , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub enum Version { # [doc = " Unknown version."] Unknown , # [doc = " Semantic version (major.minor.patch)."] Semantic (u64 , u64 , u64) , # [doc = " Rolling version. Optionally contains the release date in the string format."] Rolling (Option < String >) , # [doc = " Custom version format."] Custom (String) , }
};
}
