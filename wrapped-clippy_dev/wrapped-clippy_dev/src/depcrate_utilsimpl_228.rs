// Generated macro for impl_228 (impl)
macro_rules! Depcrate_utilsimpl_228 {
() => {
// Module: crate::utils
// Provides: {"impl_228"}
// Dependencies: {}
impl ClippyInfo { # [must_use] pub fn search_for_manifest () -> Self { let mut path = env :: current_dir () . expect ("error reading the working directory") ; let mut buf = String :: new () ; loop { path . push ("Cargo.toml") ; if let Some (mut file) = File :: open_if_exists (& path , OpenOptions :: new () . read (true)) { file . read_to_cleared_string (& mut buf) ; let package = parse_cargo_package (& buf) ; if package . name == "\"clippy\"" { if let Some (version) = buf [package . version_range] . strip_prefix ('"') && let Some (version) = version . strip_suffix ('"') && let Ok (version) = version . parse () { path . pop () ; return ClippyInfo { path , version , has_intellij_hook : ! package . not_a_platform_range . is_empty () , } ; } panic ! ("error reading clippy version from `{}`" , file . path . display ()) ; } } path . pop () ; assert ! (path . pop () , "error finding project root, please run from inside the clippy directory") ; } } }
};
}
