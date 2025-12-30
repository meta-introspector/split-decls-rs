// Generated macro for Version (enum)
macro_rules! Depcrate_dataVersion {
() => {
// Module: crate::data
// Provides: {"Version"}
// Dependencies: {}
# [doc = " Supported versions of a pack data file"] # [derive (Default , PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [allow (missing_docs)] pub enum Version { # [default] V2 , V3 , }
};
}
