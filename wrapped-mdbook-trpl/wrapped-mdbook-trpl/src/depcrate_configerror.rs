// Generated macro for Error (enum)
macro_rules! Depcrate_configError {
() => {
// Module: crate::config
// Provides: {"Error"}
// Dependencies: {}
# [derive (Debug , thiserror :: Error)] pub enum Error { # [error (transparent)] Mdbook (# [from] mdbook :: errors :: Error) , # [error ("No config for '{0}'")] NoConfig (String) , # [error ("Bad config value '{value}' for key '{key}'")] BadValue { key : String , value : String } , }
};
}
