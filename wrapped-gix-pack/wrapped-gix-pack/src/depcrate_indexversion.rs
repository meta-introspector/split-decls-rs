// Generated macro for Version (enum)
macro_rules! Depcrate_indexVersion {
() => {
// Module: crate::index
// Provides: {"Version"}
// Dependencies: {}
# [doc = " The version of an index file"] # [derive (Default , PartialEq , Eq , Ord , PartialOrd , Debug , Hash , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [allow (missing_docs)] pub enum Version { V1 = 1 , # [default] V2 = 2 , }
};
}
