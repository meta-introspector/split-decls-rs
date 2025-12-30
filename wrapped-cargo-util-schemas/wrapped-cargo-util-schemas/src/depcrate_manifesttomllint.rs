// Generated macro for TomlLint (enum)
macro_rules! Depcrate_manifestTomlLint {
() => {
// Module: crate::manifest
// Provides: {"TomlLint"}
// Dependencies: {}
# [derive (Serialize , Debug , Clone)] # [serde (untagged)] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub enum TomlLint { Level (TomlLintLevel) , Config (TomlLintConfig) , }
};
}
