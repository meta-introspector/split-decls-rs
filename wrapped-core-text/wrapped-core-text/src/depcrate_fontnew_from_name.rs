// Generated macro for new_from_name (function)
macro_rules! Depcrate_fontnew_from_name {
() => {
// Module: crate::font
// Provides: {"new_from_name"}
// Dependencies: {}
pub fn new_from_name (name : & str , pt_size : f64) -> Result < CTFont , () > { unsafe { let name : CFString = name . parse () . unwrap () ; let font_ref = CTFontCreateWithName (name . as_concrete_TypeRef () , pt_size as CGFloat , ptr :: null ()) ; if font_ref . is_null () { Err (()) } else { Ok (CTFont :: wrap_under_create_rule (font_ref)) } } }
};
}
