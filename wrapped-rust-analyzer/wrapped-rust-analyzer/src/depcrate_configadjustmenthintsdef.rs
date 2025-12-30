// Generated macro for AdjustmentHintsDef (enum)
macro_rules! Depcrate_configAdjustmentHintsDef {
() => {
// Module: crate::config
// Provides: {"AdjustmentHintsDef"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Debug , Clone)] # [serde (rename_all = "snake_case")] enum AdjustmentHintsDef { # [serde (alias = "reborrow")] Borrows , # [serde (with = "true_or_always")] # [serde (untagged)] Always , # [serde (with = "false_or_never")] # [serde (untagged)] Never , }
};
}
