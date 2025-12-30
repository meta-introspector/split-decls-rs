// Generated macro for TomlConfig (struct)
macro_rules! Depcrate_core_config_tomlTomlConfig {
() => {
// Module: crate::core::config::toml
// Provides: {"TomlConfig"}
// Dependencies: {}
# [doc = " Structure of the `bootstrap.toml` file that configuration is read from."] # [doc = ""] # [doc = " This structure uses `Decodable` to automatically decode a TOML configuration"] # [doc = " file into this format, and then this is traversed and written into the above"] # [doc = " `Config` structure."] # [derive (Deserialize , Default)] # [serde (deny_unknown_fields , rename_all = "kebab-case")] pub (crate) struct TomlConfig { # [serde (flatten)] pub (crate) change_id : ChangeIdWrapper , pub (super) build : Option < Build > , pub (super) install : Option < Install > , pub (super) llvm : Option < Llvm > , pub (super) gcc : Option < Gcc > , pub (super) rust : Option < Rust > , pub (super) target : Option < HashMap < String , TomlTarget > > , pub (super) dist : Option < Dist > , pub (super) profile : Option < String > , pub (super) include : Option < Vec < PathBuf > > , }
};
}
