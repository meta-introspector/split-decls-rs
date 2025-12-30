// Generated macro for TomlTrimPaths (enum)
macro_rules! Depcrate_manifestTomlTrimPaths {
() => {
// Module: crate::manifest
// Provides: {"TomlTrimPaths"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq , Eq , Ord , PartialOrd , Hash , Serialize)] # [serde (untagged , rename_all = "kebab-case")] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub enum TomlTrimPaths { Values (Vec < TomlTrimPathsValue >) , All , }
};
}
