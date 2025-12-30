// Generated macro for compute_predicates (function)
macro_rules! Depcrate_internals_genericscompute_predicates {
() => {
// Module: crate::internals::generics
// Provides: {"compute_predicates"}
// Dependencies: {}
pub fn compute_predicates (params : Vec < Type > , traitname : & Path) -> Vec < WherePredicate > { params . into_iter () . map (| param | { syn :: parse2 (quote ! { # param : # traitname }) . unwrap () }) . collect () }
};
}
