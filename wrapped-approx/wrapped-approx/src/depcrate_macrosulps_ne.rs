// Generated macro for ulps_ne (macro)
macro_rules! Depcrate_macrosulps_ne {
() => {
// Module: crate::macros
// Provides: {"ulps_ne"}
// Dependencies: {}
# [doc = " Approximate inequality using both the absolute difference and ULPs (Units in Last Place)."] # [macro_export] macro_rules ! ulps_ne { ($ lhs : expr , $ rhs : expr $ (, $ opt : ident = $ val : expr) *) => { $ crate :: Ulps :: default () $ (.$ opt ($ val)) *. ne (&$ lhs , &$ rhs) } ; ($ lhs : expr , $ rhs : expr $ (, $ opt : ident = $ val : expr) *,) => { $ crate :: Ulps :: default () $ (.$ opt ($ val)) *. ne (&$ lhs , &$ rhs) } ; }
};
}
