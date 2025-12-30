// Generated macro for abs_diff_ne (macro)
macro_rules! Depcrate_macrosabs_diff_ne {
() => {
// Module: crate::macros
// Provides: {"abs_diff_ne"}
// Dependencies: {}
# [doc = " Approximate inequality of using the absolute difference."] # [macro_export] macro_rules ! abs_diff_ne { ($ lhs : expr , $ rhs : expr $ (, $ opt : ident = $ val : expr) *) => { $ crate :: AbsDiff :: default () $ (.$ opt ($ val)) *. ne (&$ lhs , &$ rhs) } ; ($ lhs : expr , $ rhs : expr $ (, $ opt : ident = $ val : expr) *,) => { $ crate :: AbsDiff :: default () $ (.$ opt ($ val)) *. ne (&$ lhs , &$ rhs) } ; }
};
}
