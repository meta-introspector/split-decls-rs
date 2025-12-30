// Generated macro for new_from_name_and_options (function)
macro_rules! Depcrate_fontnew_from_name_and_options {
() => {
// Module: crate::font
// Provides: {"new_from_name_and_options"}
// Dependencies: {}
pub fn new_from_name_and_options (name : & str , pt_size : f64 , options : CTFontOptions ,) -> Result < CTFont , () > { unsafe { let name : CFString = name . parse () . unwrap () ; let font_ref = CTFontCreateWithNameAndOptions (name . as_concrete_TypeRef () , pt_size as CGFloat , ptr :: null () , options ,) ; if font_ref . is_null () { Err (()) } else { Ok (CTFont :: wrap_under_create_rule (font_ref)) } } }
};
}
