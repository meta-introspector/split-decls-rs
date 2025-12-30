// Generated macro for ImportPrefixDef (enum)
macro_rules! Depcrate_configImportPrefixDef {
() => {
// Module: crate::config
// Provides: {"ImportPrefixDef"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Debug , Clone)] # [serde (rename_all = "snake_case")] enum ImportPrefixDef { Plain , # [serde (rename = "self")] # [serde (alias = "by_self")] BySelf , # [serde (rename = "crate")] # [serde (alias = "by_crate")] ByCrate , }
};
}
