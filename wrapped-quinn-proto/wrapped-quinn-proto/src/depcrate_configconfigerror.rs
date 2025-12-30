// Generated macro for ConfigError (enum)
macro_rules! Depcrate_configConfigError {
() => {
// Module: crate::config
// Provides: {"ConfigError"}
// Dependencies: {}
# [doc = " Errors in the configuration of an endpoint"] # [derive (Debug , Error , Clone , PartialEq , Eq)] # [non_exhaustive] pub enum ConfigError { # [doc = " Value exceeds supported bounds"] # [error ("value exceeds supported bounds")] OutOfBounds , }
};
}
