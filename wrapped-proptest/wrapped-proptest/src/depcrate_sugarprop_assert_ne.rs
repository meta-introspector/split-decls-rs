// Generated macro for prop_assert_ne (macro)
macro_rules! Depcrate_sugarprop_assert_ne {
() => {
// Module: crate::sugar
// Provides: {"prop_assert_ne"}
// Dependencies: {}
# [doc = " Similar to `assert_ne!` from std, but returns a test failure instead of"] # [doc = " panicking if the condition fails."] # [doc = ""] # [doc = " See `prop_assert!` for a more in-depth discussion."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```"] # [doc = " use proptest::prelude::*;"] # [doc = ""] # [doc = " proptest! {"] # [doc = "   # /*"] # [doc = "   #[test]"] # [doc = "   # */"] # [doc = "   fn test_addition(a in 0i32..100i32, b in 1i32..100i32) {"] # [doc = "     // Use with default message"] # [doc = "     prop_assert_ne!(a, a + b);"] # [doc = "     // Can also provide custom message added after the common message"] # [doc = "     prop_assert_ne!(a, a + b, \"a = {}, b = {}\", a, b);"] # [doc = "   }"] # [doc = " }"] # [doc = " #"] # [doc = " # fn main() { test_addition(); }"] # [doc = " ```"] # [macro_export] macro_rules ! prop_assert_ne { ($ left : expr , $ right : expr $ (,) ?) => { { let left = $ left ; let right = $ right ; $ crate :: prop_assert ! (left != right , "assertion failed: `(left != right)`\
             \n  left: `{:?}`,\n right: `{:?}`" , left , right) ; } } ; ($ left : expr , $ right : expr , $ fmt : tt $ ($ args : tt) *) => { { let left = $ left ; let right = $ right ; $ crate :: prop_assert ! (left != right , concat ! ("assertion failed: `(left != right)`\
                 \n  left: `{:?}`,\n right: `{:?}`: " , $ fmt) , left , right $ ($ args) *) ; } } ; }
};
}
