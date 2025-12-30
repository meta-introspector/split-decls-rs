// Generated macro for impl_22 (impl)
macro_rules! Depcrate_format_manimpl_22 {
() => {
// Module: crate::format::man
// Provides: {"impl_22"}
// Dependencies: {}
impl Font { fn str_from_stack (font_stack : & [Font]) -> & 'static str { let has_bold = font_stack . iter () . any (| font | matches ! (font , Font :: Bold)) ; let has_italic = font_stack . iter () . any (| font | matches ! (font , Font :: Italic)) ; match (has_bold , has_italic) { (false , false) => "\\fR" , (false , true) => "\\fI" , (true , false) => "\\fB" , (true , true) => "\\f(BI" , } } }
};
}
