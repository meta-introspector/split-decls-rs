// Generated macro for impl_991 (impl)
macro_rules! Depcrate_themeimpl_991 {
() => {
// Module: crate::theme
// Provides: {"impl_991"}
// Dependencies: {}
impl Options { pub fn to_theme (& self , isatty : bool) -> Theme { if self . use_colours == UseColours :: Never || (self . use_colours == UseColours :: Automatic && ! isatty) { let ui = UiStyles :: plain () ; let exts = Box :: new (NoFileStyle) ; return Theme { ui , exts } ; } ; # [cfg (windows)] if nu_ansi_term :: enable_ansi_support () . is_err () { if self . use_colours == UseColours :: Always { eprintln ! ("eza: Ignoring option color=always in legacy console.") ; } let ui = UiStyles :: plain () ; let exts = Box :: new (NoFileStyle) ; return Theme { ui , exts } ; } match self . theme_config { Some (ref theme) => { if let Some (mut ui) = theme . to_theme () { let (exts , use_default_filetypes) = self . definitions . parse_color_vars (& mut ui) ; let exts : Box < dyn FileStyle > = match (exts . is_non_empty () , use_default_filetypes) { (false , false) => Box :: new (NoFileStyle) , (false , true) => Box :: new (FileTypes) , (true , false) => Box :: new (exts) , (true , true) => Box :: new ((exts , FileTypes)) , } ; return Theme { ui , exts } ; } self . default_theme () } None => self . default_theme () , } } fn default_theme (& self) -> Theme { let mut ui = UiStyles :: default_theme (self . colour_scale) ; let (exts , use_default_filetypes) = self . definitions . parse_color_vars (& mut ui) ; let exts : Box < dyn FileStyle > = match (exts . is_non_empty () , use_default_filetypes) { (false , false) => Box :: new (NoFileStyle) , (false , true) => Box :: new (FileTypes) , (true , false) => Box :: new (exts) , (true , true) => Box :: new ((exts , FileTypes)) , } ; Theme { ui , exts } } }
};
}
