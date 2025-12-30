// Generated macro for assert_eq (macro)
macro_rules! Depcrateassert_eq {
() => {
// Module: crate
// Provides: {"assert_eq"}
// Dependencies: {}
# [doc = " Asserts that two expressions are equal to each other (using [`PartialEq`])."] # [doc = ""] # [doc = " On panic, this macro will print a diff derived from [`Debug`] representation of"] # [doc = " each value."] # [doc = ""] # [doc = " This is a drop in replacement for [`core::assert_eq!`]."] # [doc = " You can provide a custom panic message if desired."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use pretty_assertions::assert_eq;"] # [doc = ""] # [doc = " let a = 3;"] # [doc = " let b = 1 + 2;"] # [doc = " assert_eq!(a, b);"] # [doc = ""] # [doc = " assert_eq!(a, b, \"we are testing addition with {} and {}\", a, b);"] # [doc = " ```"] # [macro_export] macro_rules ! assert_eq { ($ left : expr , $ right : expr $ (,) ?) => ({ $ crate :: assert_eq ! (@ $ left , $ right , "" , "") ; }) ; ($ left : expr , $ right : expr , $ ($ arg : tt) *) => ({ $ crate :: assert_eq ! (@ $ left , $ right , ": " , $ ($ arg) +) ; }) ; (@ $ left : expr , $ right : expr , $ maybe_colon : expr , $ ($ arg : tt) *) => ({ match (& ($ left) , & ($ right)) { (left_val , right_val) => { if ! (* left_val == * right_val) { use $ crate :: private :: CreateComparison ; :: core :: panic ! ("assertion failed: `(left == right)`{}{}\
                       \n\
                       \n{}\
                       \n" , $ maybe_colon , format_args ! ($ ($ arg) *) , (left_val , right_val) . create_comparison ()) } } } }) ; }
};
}
