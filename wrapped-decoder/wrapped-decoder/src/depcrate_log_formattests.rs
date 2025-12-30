// Generated macro for tests (module)
macro_rules! Depcrate_log_formattests {
() => {
// Module: crate::log::format
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_left_aligned_styled_string () { let format = LogFormat { color : Some (LogColor :: Color (Color :: Green)) , width : Some (10) , alignment : Some (Alignment :: Left) , padding : Some (Padding :: Space) , style : Some (vec ! [Styles :: Bold]) , } ; let s = build_formatted_string ("test" , & format , 0 , None , None) ; let string_without_styles = string_excluding_ansi (& s) ; assert_eq ! (string_without_styles , "test      ") ; } # [test] fn test_right_aligned_styled_string () { let format = LogFormat { color : Some (LogColor :: Color (Color :: Green)) , width : Some (10) , alignment : Some (Alignment :: Right) , padding : Some (Padding :: Space) , style : Some (vec ! [Styles :: Bold]) , } ; let s = build_formatted_string ("test" , & format , 0 , None , None) ; let string_without_styles = string_excluding_ansi (& s) ; assert_eq ! (string_without_styles , "      test") ; } }
};
}
