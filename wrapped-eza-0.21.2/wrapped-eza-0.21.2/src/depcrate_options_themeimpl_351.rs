// Generated macro for impl_351 (impl)
macro_rules! Depcrate_options_themeimpl_351 {
() => {
// Module: crate::options::theme
// Provides: {"impl_351"}
// Dependencies: {}
impl Options { pub fn deduce < V : Vars > (matches : & MatchedFlags < '_ > , vars : & V) -> Result < Self , OptionsError > { let use_colours = UseColours :: deduce (matches , vars) ? ; let colour_scale = ColorScaleOptions :: deduce (matches , vars) ? ; let theme_config = ThemeConfig :: deduce (vars) ; let definitions = if use_colours == UseColours :: Never { Definitions :: default () } else { Definitions :: deduce (vars) } ; Ok (Self { use_colours , colour_scale , definitions , theme_config , }) } }
};
}
