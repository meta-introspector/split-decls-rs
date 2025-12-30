// Generated macro for TomlLintLevel (enum)
macro_rules! Depcrate_manifestTomlLintLevel {
() => {
// Module: crate::manifest
// Provides: {"TomlLintLevel"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Debug , Copy , Clone , Eq , PartialEq)] # [serde (rename_all = "kebab-case")] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub enum TomlLintLevel { Forbid , Deny , Warn , Allow , }
};
}
