// Generated macro for version_meta (function)
macro_rules! Depcrateversion_meta {
() => {
// Module: crate
// Provides: {"version_meta"}
// Dependencies: {}
# [doc = " Returns the `rustc` SemVer version and additional metadata"] # [doc = " like the git short hash and build date."] pub fn version_meta () -> Result < VersionMeta > { let rustc = env :: var_os ("RUSTC") . unwrap_or_else (| | OsString :: from ("rustc")) ; let cmd = if let Some (wrapper) = env :: var_os ("RUSTC_WRAPPER") . filter (| w | ! w . is_empty ()) { let mut cmd = Command :: new (wrapper) ; cmd . arg (rustc) ; cmd } else { Command :: new (rustc) } ; VersionMeta :: for_command (cmd) }
};
}
