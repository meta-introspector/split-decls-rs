// Generated macro for LifetimeElisionDef (enum)
macro_rules! Depcrate_configLifetimeElisionDef {
() => {
// Module: crate::config
// Provides: {"LifetimeElisionDef"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Debug , Clone)] # [serde (rename_all = "snake_case")] enum LifetimeElisionDef { SkipTrivial , # [serde (with = "true_or_always")] # [serde (untagged)] Always , # [serde (with = "false_or_never")] # [serde (untagged)] Never , }
};
}
