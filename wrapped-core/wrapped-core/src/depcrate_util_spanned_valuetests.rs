// Generated macro for tests (module)
macro_rules! Depcrate_util_spanned_valuetests {
() => {
// Module: crate::util::spanned_value
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use proc_macro2 :: Span ; # [doc = " Make sure that `SpannedValue` can be seamlessly used as its underlying type."] # [test] fn deref () { let test = SpannedValue :: new ("hello" , Span :: call_site ()) ; assert_eq ! ("hello" , test . trim ()) ; } }
};
}
