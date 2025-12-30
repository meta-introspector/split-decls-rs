// Generated macro for ConfigErrorInner (enum)
macro_rules! Depcrate_configConfigErrorInner {
() => {
// Module: crate::config
// Provides: {"ConfigErrorInner"}
// Dependencies: {}
# [derive (Debug)] pub enum ConfigErrorInner { Json { config_key : String , error : serde_json :: Error } , Toml { config_key : String , error : toml :: de :: Error } , ParseError { reason : String } , }
};
}
