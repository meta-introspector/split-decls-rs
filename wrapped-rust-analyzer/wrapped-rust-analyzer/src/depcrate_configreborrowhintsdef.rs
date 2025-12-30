// Generated macro for ReborrowHintsDef (enum)
macro_rules! Depcrate_configReborrowHintsDef {
() => {
// Module: crate::config
// Provides: {"ReborrowHintsDef"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Debug , Clone)] # [serde (rename_all = "snake_case")] enum ReborrowHintsDef { Mutable , # [serde (with = "true_or_always")] # [serde (untagged)] Always , # [serde (with = "false_or_never")] # [serde (untagged)] Never , }
};
}
