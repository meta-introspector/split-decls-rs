// Generated macro for impl_468 (impl)
macro_rules! Depcrate_options_configimpl_468 {
() => {
// Module: crate::options::config
// Provides: {"impl_468"}
// Dependencies: {}
impl Default for ThemeConfig { fn default () -> Self { ThemeConfig { location : dirs :: config_dir () . unwrap_or_default () . join ("eza") . join ("theme.yml") , } } }
};
}
