// Generated macro for ulps_eq (macro)
macro_rules! Depcrate_macrosulps_eq {
() => {
// Module: crate::macros
// Provides: {"ulps_eq"}
// Dependencies: {}
# [doc = " Approximate equality using both the absolute difference and ULPs (Units in Last Place)."] # [macro_export] macro_rules ! ulps_eq { ($ lhs : expr , $ rhs : expr $ (, $ opt : ident = $ val : expr) *) => { $ crate :: Ulps :: default () $ (.$ opt ($ val)) *. eq (&$ lhs , &$ rhs) } ; ($ lhs : expr , $ rhs : expr $ (, $ opt : ident = $ val : expr) *,) => { $ crate :: Ulps :: default () $ (.$ opt ($ val)) *. eq (&$ lhs , &$ rhs) } ; }
};
}
