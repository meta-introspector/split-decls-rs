// Generated macro for Validate (trait)
macro_rules! Depcrate_config_tree_keysValidate {
() => {
// Module: crate::config::tree::keys
// Provides: {"Validate"}
// Dependencies: {}
# [doc = " Provide a way to validate a value, or decode a value from `git-config`."] pub trait Validate { # [doc = " Validate `value` or return an error."] fn validate (& self , value : & BStr) -> Result < () , Box < dyn Error + Send + Sync + 'static > > ; }
};
}
