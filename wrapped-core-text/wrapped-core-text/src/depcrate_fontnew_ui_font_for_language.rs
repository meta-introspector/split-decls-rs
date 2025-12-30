// Generated macro for new_ui_font_for_language (function)
macro_rules! Depcrate_fontnew_ui_font_for_language {
() => {
// Module: crate::font
// Provides: {"new_ui_font_for_language"}
// Dependencies: {}
pub fn new_ui_font_for_language (ui_type : CTFontUIFontType , size : f64 , language : Option < CFString > ,) -> CTFont { unsafe { let font_ref = CTFontCreateUIFontForLanguage (ui_type , size as CGFloat , language . as_ref () . map (| x | x . as_concrete_TypeRef ()) . unwrap_or (std :: ptr :: null ()) ,) ; if font_ref . is_null () { panic ! () ; } else { CTFont :: wrap_under_create_rule (font_ref) } } }
};
}
