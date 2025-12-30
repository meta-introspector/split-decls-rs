// Generated macro for get_toml (function)
macro_rules! Depcrate_core_config_testsget_toml {
() => {
// Module: crate::core::config::tests
// Provides: {"get_toml"}
// Dependencies: {}
fn get_toml (file : & Path) -> Result < TomlConfig , toml :: de :: Error > { let contents = std :: fs :: read_to_string (file) . unwrap () ; toml :: from_str (& contents) . and_then (| table : toml :: Value | TomlConfig :: deserialize (table)) }
};
}
