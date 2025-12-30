// Generated macro for prop_assert_eq (macro)
macro_rules! Depcrate_sugarprop_assert_eq {
() => {
// Module: crate::sugar
// Provides: {"prop_assert_eq"}
// Dependencies: {}
# [doc = " Similar to `assert_eq!` from std, but returns a test failure instead of"] # [doc = " panicking if the condition fails."] # [doc = ""] # [doc = " See `prop_assert!` for a more in-depth discussion."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```"] # [doc = " use proptest::prelude::*;"] # [doc = ""] # [doc = " proptest! {"] # [doc = "   # /*"] # [doc = "   #[test]"] # [doc = "   # */"] # [doc = "   fn concat_string_length(ref a in \".*\", ref b in \".*\") {"] # [doc = "     let cat = format!(\"{}{}\", a, b);"] # [doc = "     // Use with default message"] # [doc = "     prop_assert_eq!(a.len() + b.len(), cat.len());"] # [doc = "     // Can also provide custom message (added after the normal"] # [doc = "     // assertion message)"] # [doc = "     prop_assert_eq!(a.len() + b.len(), cat.len(),"] # [doc = "                     \"a = {:?}, b = {:?}\", a, b);"] # [doc = "   }"] # [doc = " }"] # [doc = " #"] # [doc = " # fn main() { concat_string_length(); }"] # [doc = " ```"] # [macro_export] macro_rules ! prop_assert_eq { ($ left : expr , $ right : expr $ (,) ?) => { { let left = $ left ; let right = $ right ; $ crate :: prop_assert ! (left == right , "assertion failed: `(left == right)` \
             \n  left: `{:?}`,\n right: `{:?}`" , left , right) ; } } ; ($ left : expr , $ right : expr , $ fmt : tt $ ($ args : tt) *) => { { let left = $ left ; let right = $ right ; $ crate :: prop_assert ! (left == right , concat ! ("assertion failed: `(left == right)` \
                 \n  left: `{:?}`, \n right: `{:?}`: " , $ fmt) , left , right $ ($ args) *) ; } } ; }
};
}
