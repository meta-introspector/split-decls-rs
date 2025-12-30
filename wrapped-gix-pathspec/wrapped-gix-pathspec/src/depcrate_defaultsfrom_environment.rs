// Generated macro for from_environment (module)
macro_rules! Depcrate_defaultsfrom_environment {
() => {
// Module: crate::defaults
// Provides: {"from_environment"}
// Dependencies: {}
# [doc = ""] pub mod from_environment { # [doc = " The error returned by [Defaults::from_environment()](super::Defaults::from_environment())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] ParseValue (# [from] gix_config_value :: Error) , # [error ("Glob and no-glob settings are mutually exclusive")] MixedGlobAndNoGlob , } }
};
}
