// Generated macro for assert_ne (macro)
macro_rules! Depcrateassert_ne {
() => {
// Module: crate
// Provides: {"assert_ne"}
// Dependencies: {}
# [doc = " Asserts that two expressions are not equal to each other (using [`PartialEq`])."] # [doc = ""] # [doc = " On panic, this macro will print the values of the expressions with their"] # [doc = " [`Debug`] representations."] # [doc = ""] # [doc = " This is a drop in replacement for [`core::assert_ne!`]."] # [doc = " You can provide a custom panic message if desired."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use pretty_assertions::assert_ne;"] # [doc = ""] # [doc = " let a = 3;"] # [doc = " let b = 2;"] # [doc = " assert_ne!(a, b);"] # [doc = ""] # [doc = " assert_ne!(a, b, \"we are testing that the values are not equal\");"] # [doc = " ```"] # [macro_export] macro_rules ! assert_ne { ($ left : expr , $ right : expr $ (,) ?) => ({ $ crate :: assert_ne ! (@ $ left , $ right , "" , "") ; }) ; ($ left : expr , $ right : expr , $ ($ arg : tt) +) => ({ $ crate :: assert_ne ! (@ $ left , $ right , ": " , $ ($ arg) +) ; }) ; (@ $ left : expr , $ right : expr , $ maybe_colon : expr , $ ($ arg : tt) +) => ({ match (& ($ left) , & ($ right)) { (left_val , right_val) => { if * left_val == * right_val { :: core :: panic ! ("assertion failed: `(left != right)`{}{}\
                        \n\
                        \nBoth sides:\
                        \n{:#?}\
                        \n\
                        \n" , $ maybe_colon , format_args ! ($ ($ arg) +) , left_val) } } } }) ; }
};
}
