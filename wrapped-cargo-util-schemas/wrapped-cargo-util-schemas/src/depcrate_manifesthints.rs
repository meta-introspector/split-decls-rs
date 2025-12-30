// Generated macro for Hints (struct)
macro_rules! Depcrate_manifestHints {
() => {
// Module: crate::manifest
// Provides: {"Hints"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Debug , Default , Clone)] # [serde (rename_all = "kebab-case")] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub struct Hints { # [cfg_attr (feature = "unstable-schema" , schemars (with = "Option<TomlValueWrapper>"))] pub mostly_unused : Option < toml :: Value > , }
};
}
