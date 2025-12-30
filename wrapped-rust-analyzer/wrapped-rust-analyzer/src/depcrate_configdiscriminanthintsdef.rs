// Generated macro for DiscriminantHintsDef (enum)
macro_rules! Depcrate_configDiscriminantHintsDef {
() => {
// Module: crate::config
// Provides: {"DiscriminantHintsDef"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Debug , Clone)] # [serde (rename_all = "snake_case")] enum DiscriminantHintsDef { Fieldless , # [serde (with = "true_or_always")] # [serde (untagged)] Always , # [serde (with = "false_or_never")] # [serde (untagged)] Never , }
};
}
