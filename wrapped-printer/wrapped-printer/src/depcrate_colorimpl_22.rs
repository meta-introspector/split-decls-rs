// Generated macro for impl_22 (impl)
macro_rules! Depcrate_colorimpl_22 {
() => {
// Module: crate::color
// Provides: {"impl_22"}
// Dependencies: {}
impl SpecValue { # [doc = " Merge this spec value into the given color specification."] fn merge_into (& self , cspec : & mut ColorSpec) { match * self { SpecValue :: None => cspec . clear () , SpecValue :: Fg (ref color) => { cspec . set_fg (Some (color . clone ())) ; } SpecValue :: Bg (ref color) => { cspec . set_bg (Some (color . clone ())) ; } SpecValue :: Style (ref style) => match * style { Style :: Bold => { cspec . set_bold (true) ; } Style :: NoBold => { cspec . set_bold (false) ; } Style :: Intense => { cspec . set_intense (true) ; } Style :: NoIntense => { cspec . set_intense (false) ; } Style :: Underline => { cspec . set_underline (true) ; } Style :: NoUnderline => { cspec . set_underline (false) ; } Style :: Italic => { cspec . set_italic (true) ; } Style :: NoItalic => { cspec . set_italic (false) ; } } , } } }
};
}
