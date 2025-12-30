// Generated macro for determine_derive_macro (function)
macro_rules! Depcrate_derivable_implsdetermine_derive_macro {
() => {
// Module: crate::derivable_impls
// Provides: {"determine_derive_macro"}
// Dependencies: {}
fn determine_derive_macro (cx : & LateContext < '_ > , is_const : bool) -> Option < & 'static str > { (! is_const) . then_some ("derive") . or_else (| | cx . tcx . features () . enabled (sym :: derive_const) . then_some ("derive_const")) }
};
}
