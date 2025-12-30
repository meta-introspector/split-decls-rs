// Generated macro for impl_503 (impl)
macro_rules! Depcrate_options_configimpl_503 {
() => {
// Module: crate::options::config
// Provides: {"impl_503"}
// Dependencies: {}
impl ThemeConfig { pub fn from_path (path : PathBuf) -> Self { ThemeConfig { location : path } } pub fn to_theme (& self) -> Option < UiStyles > { let ui_styles_override : Option < UiStylesOverride > = { let file = std :: fs :: File :: open (& self . location) . ok () ? ; serde_norway :: from_reader (& file) . ok () } ; FromOverride :: from (ui_styles_override , Some (UiStyles :: default ())) } }
};
}
