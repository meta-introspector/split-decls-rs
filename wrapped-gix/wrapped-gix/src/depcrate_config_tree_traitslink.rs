// Generated macro for Link (enum)
macro_rules! Depcrate_config_tree_traitsLink {
() => {
// Module: crate::config::tree::traits
// Provides: {"Link"}
// Dependencies: {}
# [doc = " A way to link a key with other resources."] # [derive (Debug , Copy , Clone)] pub enum Link { # [doc = " The environment variable of the given name will override the value of this key."] EnvironmentOverride (& 'static str) , # [doc = " This config key is used as fallback if this key isn't set."] FallbackKey (& 'static dyn Key) , }
};
}
