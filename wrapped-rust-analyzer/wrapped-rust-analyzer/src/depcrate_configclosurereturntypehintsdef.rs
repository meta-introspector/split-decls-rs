// Generated macro for ClosureReturnTypeHintsDef (enum)
macro_rules! Depcrate_configClosureReturnTypeHintsDef {
() => {
// Module: crate::config
// Provides: {"ClosureReturnTypeHintsDef"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Debug , Clone)] # [serde (rename_all = "snake_case")] enum ClosureReturnTypeHintsDef { WithBlock , # [serde (with = "true_or_always")] # [serde (untagged)] Always , # [serde (with = "false_or_never")] # [serde (untagged)] Never , }
};
}
