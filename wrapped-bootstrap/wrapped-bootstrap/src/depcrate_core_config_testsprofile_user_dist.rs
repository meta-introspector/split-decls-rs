// Generated macro for profile_user_dist (function)
macro_rules! Depcrate_core_config_testsprofile_user_dist {
() => {
// Module: crate::core::config::tests
// Provides: {"profile_user_dist"}
// Dependencies: {}
# [test] fn profile_user_dist () { fn get_toml (file : & Path) -> Result < TomlConfig , toml :: de :: Error > { let contents = if file . ends_with ("bootstrap.toml") || file . ends_with ("config.toml") || env :: var_os ("RUST_BOOTSTRAP_CONFIG") . is_some () { "profile = \"user\"" . to_owned () } else { assert ! (file . ends_with ("config.dist.toml") || file . ends_with ("bootstrap.dist.toml")) ; std :: fs :: read_to_string (file) . unwrap () } ; toml :: from_str (& contents) . and_then (| table : toml :: Value | TomlConfig :: deserialize (table)) } Config :: parse_inner (Flags :: parse (& ["check" . to_owned ()]) , get_toml) ; }
};
}
