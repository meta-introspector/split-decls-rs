// Generated macro for ReplaceOpt (enum)
macro_rules! Depcrate_core_configReplaceOpt {
() => {
// Module: crate::core::config
// Provides: {"ReplaceOpt"}
// Dependencies: {}
# [doc = " Describes how to handle conflicts in merging two `TomlConfig`"] # [derive (Copy , Clone , Debug)] pub enum ReplaceOpt { # [doc = " Silently ignore a duplicated value"] IgnoreDuplicate , # [doc = " Override the current value, even if it's `Some`"] Override , # [doc = " Exit with an error on duplicate values"] ErrorOnDuplicate , }
};
}
