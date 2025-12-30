// Generated macro for load_toml_config (function)
macro_rules! Depcrate_core_config_configload_toml_config {
() => {
// Module: crate::core::config::config
// Provides: {"load_toml_config"}
// Dependencies: {}
# [doc = " Loads bootstrap TOML config and returns the config together with a path from where"] # [doc = " it was loaded."] # [doc = " `src` is the source root directory, and `config_path` is an optionally provided path to the"] # [doc = " config."] fn load_toml_config (src : & Path , config_path : Option < PathBuf > , get_toml : & impl Fn (& Path) -> Result < TomlConfig , toml :: de :: Error > ,) -> (TomlConfig , Option < PathBuf >) { let toml_path = config_path . or_else (| | env :: var_os ("RUST_BOOTSTRAP_CONFIG") . map (PathBuf :: from)) ; let using_default_path = toml_path . is_none () ; let mut toml_path = toml_path . unwrap_or_else (| | PathBuf :: from ("bootstrap.toml")) ; if using_default_path && ! toml_path . exists () { toml_path = src . join (PathBuf :: from ("bootstrap.toml")) ; if ! toml_path . exists () { toml_path = PathBuf :: from ("config.toml") ; if ! toml_path . exists () { toml_path = src . join (PathBuf :: from ("config.toml")) ; } } } if ! using_default_path || toml_path . exists () { let path = Some (if cfg ! (not (test)) { toml_path = toml_path . canonicalize () . unwrap () ; toml_path . clone () } else { toml_path . clone () }) ; (get_toml (& toml_path) . unwrap_or_else (| e | { eprintln ! ("ERROR: Failed to parse '{}': {e}" , toml_path . display ()) ; exit ! (2) ; }) , path ,) } else { (TomlConfig :: default () , None) } }
};
}
