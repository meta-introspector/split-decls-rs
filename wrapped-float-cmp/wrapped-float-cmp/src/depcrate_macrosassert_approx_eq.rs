// Generated macro for assert_approx_eq (macro)
macro_rules! Depcrate_macrosassert_approx_eq {
() => {
// Module: crate::macros
// Provides: {"assert_approx_eq"}
// Dependencies: {}
# [macro_export] macro_rules ! assert_approx_eq { ($ typ : ty , $ lhs : expr , $ rhs : expr) => { { match (&$ lhs , &$ rhs) { (left_val , right_val) => { if !$ crate :: approx_eq ! ($ typ , * left_val , * right_val) { panic ! (r#"assertion failed: `(left approx_eq right)`
  left: `{:?}`,
 right: `{:?}`"# , left_val , right_val ,) } } } } } ; ($ typ : ty , $ lhs : expr , $ rhs : expr $ (, $ set : ident = $ val : expr) *) => { { match (&$ lhs , &$ rhs) { (left_val , right_val) => { if !$ crate :: approx_eq ! ($ typ , * left_val , * right_val $ (, $ set = $ val) *) { panic ! (r#"assertion failed: `(left approx_eq right)`
  left: `{:?}`,
 right: `{:?}`"# , left_val , right_val ,) } } } } } ; ($ typ : ty , $ lhs : expr , $ rhs : expr , $ marg : expr) => { { match (&$ lhs , &$ rhs) { (left_val , right_val) => { if !$ crate :: approx_eq ! ($ typ , * left_val , * right_val , $ marg) { panic ! (r#"assertion failed: `(left approx_eq right)`
  left: `{:?}`,
 right: `{:?}`"# , left_val , right_val ,) } } } } } ; }
};
}
