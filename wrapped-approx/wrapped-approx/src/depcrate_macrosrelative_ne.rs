// Generated macro for relative_ne (macro)
macro_rules! Depcrate_macrosrelative_ne {
() => {
// Module: crate::macros
// Provides: {"relative_ne"}
// Dependencies: {}
# [doc = " Approximate inequality using both the absolute difference and relative based comparisons."] # [macro_export] macro_rules ! relative_ne { ($ lhs : expr , $ rhs : expr $ (, $ opt : ident = $ val : expr) *) => { $ crate :: Relative :: default () $ (.$ opt ($ val)) *. ne (&$ lhs , &$ rhs) } ; ($ lhs : expr , $ rhs : expr $ (, $ opt : ident = $ val : expr) *,) => { $ crate :: Relative :: default () $ (.$ opt ($ val)) *. ne (&$ lhs , &$ rhs) } ; }
};
}
