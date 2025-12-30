// Generated macro for Version (enum)
macro_rules! DepcrateVersion {
() => {
// Module: crate
// Provides: {"Version"}
// Dependencies: {}
# [doc = " All known versions of a git index file."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum Version { # [doc = " Supports entries and various extensions."] V2 = 2 , # [doc = " Adds support for additional flags for each entry, called extended entries."] V3 = 3 , # [doc = " Supports deltified entry paths."] V4 = 4 , }
};
}
