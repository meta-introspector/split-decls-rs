// Generated macro for impl_1186 (impl)
macro_rules! Depcrate_core_config_tomlimpl_1186 {
() => {
// Module: crate::core::config::toml
// Provides: {"impl_1186"}
// Dependencies: {}
impl Config { pub (crate) fn get_builder_toml (& self , build_name : & str) -> Result < TomlConfig , toml :: de :: Error > { if self . dry_run () { return Ok (TomlConfig :: default ()) ; } let builder_config_path = self . out . join (self . host_target . triple) . join (build_name) . join (BUILDER_CONFIG_FILENAME) ; Self :: get_toml (& builder_config_path) } pub (crate) fn get_toml (file : & Path) -> Result < TomlConfig , toml :: de :: Error > { # [cfg (test)] return Ok (TomlConfig :: default ()) ; # [cfg (not (test))] Self :: get_toml_inner (file) } pub (crate) fn get_toml_inner (file : & Path) -> Result < TomlConfig , toml :: de :: Error > { let contents = t ! (fs :: read_to_string (file) , format ! ("config file {} not found" , file . display ())) ; toml :: from_str (& contents) . and_then (| table : toml :: Value | TomlConfig :: deserialize (table)) . inspect_err (| _ | { if let Ok (ChangeIdWrapper { inner : Some (ChangeId :: Id (id)) }) = toml :: from_str :: < toml :: Value > (& contents) . and_then (| table : toml :: Value | ChangeIdWrapper :: deserialize (table)) { let changes = crate :: find_recent_config_change_ids (id) ; if ! changes . is_empty () { println ! ("WARNING: There have been changes to x.py since you last updated:\n{}" , crate :: human_readable_changes (changes)) ; } } }) } }
};
}
