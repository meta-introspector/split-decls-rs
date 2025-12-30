// Generated macro for relative_eq (macro)
macro_rules! Depcrate_macrosrelative_eq {
() => {
// Module: crate::macros
// Provides: {"relative_eq"}
// Dependencies: {}
# [doc = " Approximate equality using both the absolute difference and relative based comparisons."] # [macro_export] macro_rules ! relative_eq { ($ lhs : expr , $ rhs : expr $ (, $ opt : ident = $ val : expr) *) => { $ crate :: Relative :: default () $ (.$ opt ($ val)) *. eq (&$ lhs , &$ rhs) } ; ($ lhs : expr , $ rhs : expr $ (, $ opt : ident = $ val : expr) *,) => { $ crate :: Relative :: default () $ (.$ opt ($ val)) *. eq (&$ lhs , &$ rhs) } ; }
};
}
