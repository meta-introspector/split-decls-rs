// Generated macro for TomlLintConfig (struct)
macro_rules! Depcrate_manifestTomlLintConfig {
() => {
// Module: crate::manifest
// Provides: {"TomlLintConfig"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Debug , Clone)] # [serde (rename_all = "kebab-case")] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub struct TomlLintConfig { pub level : TomlLintLevel , # [serde (default)] pub priority : i8 , # [serde (flatten)] # [cfg_attr (feature = "unstable-schema" , schemars (with = "HashMap<String, TomlValueWrapper>"))] pub config : toml :: Table , }
};
}
