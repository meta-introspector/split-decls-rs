// Generated macro for assert_str_eq (macro)
macro_rules! Depcrateassert_str_eq {
() => {
// Module: crate
// Provides: {"assert_str_eq"}
// Dependencies: {}
# [doc = " Asserts that two expressions are equal to each other (using [`PartialEq`])."] # [doc = ""] # [doc = " On panic, this macro will print a diff derived from each value's [`str`] representation."] # [doc = " See [`StrComparison`] for further details."] # [doc = ""] # [doc = " This is a drop in replacement for [`core::assert_eq!`]."] # [doc = " You can provide a custom panic message if desired."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use pretty_assertions::assert_str_eq;"] # [doc = ""] # [doc = " let a = \"foo\\nbar\";"] # [doc = " let b = [\"foo\", \"bar\"].join(\"\\n\");"] # [doc = " assert_str_eq!(a, b);"] # [doc = ""] # [doc = " assert_str_eq!(a, b, \"we are testing concatenation with {} and {}\", a, b);"] # [doc = " ```"] # [macro_export] macro_rules ! assert_str_eq { ($ left : expr , $ right : expr $ (,) ?) => ({ $ crate :: assert_str_eq ! (@ $ left , $ right , "" , "") ; }) ; ($ left : expr , $ right : expr , $ ($ arg : tt) *) => ({ $ crate :: assert_str_eq ! (@ $ left , $ right , ": " , $ ($ arg) +) ; }) ; (@ $ left : expr , $ right : expr , $ maybe_colon : expr , $ ($ arg : tt) *) => ({ match (& ($ left) , & ($ right)) { (left_val , right_val) => { if ! (* left_val == * right_val) { :: core :: panic ! ("assertion failed: `(left == right)`{}{}\
                       \n\
                       \n{}\
                       \n" , $ maybe_colon , format_args ! ($ ($ arg) *) , $ crate :: StrComparison :: new (left_val , right_val)) } } } }) ; }
};
}
