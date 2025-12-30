// Generated macro for impl_352 (impl)
macro_rules! Depcrate_options_themeimpl_352 {
() => {
// Module: crate::options::theme
// Provides: {"impl_352"}
// Dependencies: {}
impl ThemeConfig { fn deduce < V : Vars > (vars : & V) -> Option < Self > { if let Some (path) = vars . get ("EZA_CONFIG_DIR") { let path = PathBuf :: from (path) ; let theme = path . join ("theme.yml") ; if theme . exists () { return Some (ThemeConfig :: from_path (theme)) ; } let theme = path . join ("theme.yaml") ; if theme . exists () { return Some (ThemeConfig :: from_path (theme)) ; } None } else { let path = dirs :: config_dir () . unwrap_or_default () ; let path = path . join ("eza") ; let theme = path . join ("theme.yml") ; if theme . exists () { return Some (ThemeConfig :: default ()) ; } let theme = path . join ("theme.yaml") ; if theme . exists () { return Some (ThemeConfig :: from_path (theme)) ; } None } } }
};
}
