// Generated macro for abs_diff_eq (macro)
macro_rules! Depcrate_macrosabs_diff_eq {
() => {
// Module: crate::macros
// Provides: {"abs_diff_eq"}
// Dependencies: {}
# [doc = " Approximate equality of using the absolute difference."] # [macro_export] macro_rules ! abs_diff_eq { ($ lhs : expr , $ rhs : expr $ (, $ opt : ident = $ val : expr) *) => { $ crate :: AbsDiff :: default () $ (.$ opt ($ val)) *. eq (&$ lhs , &$ rhs) } ; ($ lhs : expr , $ rhs : expr $ (, $ opt : ident = $ val : expr) *,) => { $ crate :: AbsDiff :: default () $ (.$ opt ($ val)) *. eq (&$ lhs , &$ rhs) } ; }
};
}
