// Generated macro for as_static_str (function)
macro_rules! Depcrate_converteras_static_str {
() => {
// Module: crate::converter
// Provides: {"as_static_str"}
// Dependencies: {}
# [test] fn as_static_str () { use std :: borrow :: Cow ; assert_eq ! (AsStaticStr (Cow ::< str >:: Owned ("hi" . to_string ())) . bake (& Default :: default ()) . to_string () , r#""hi""#) ; }
};
}
