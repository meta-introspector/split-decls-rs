// Generated macro for InheritableLints (struct)
macro_rules! Depcrate_manifestInheritableLints {
() => {
// Module: crate::manifest
// Provides: {"InheritableLints"}
// Dependencies: {}
# [derive (Serialize , Debug , Clone)] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub struct InheritableLints { # [serde (skip_serializing_if = "std::ops::Not::not")] # [cfg_attr (feature = "unstable-schema" , schemars (default))] pub workspace : bool , # [serde (flatten)] pub lints : TomlLints , }
};
}
