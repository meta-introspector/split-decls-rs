// Generated macro for chain (macro)
macro_rules! Depcrate_utils_authorchain {
() => {
// Module: crate::utils::author
// Provides: {"chain"}
// Dependencies: {}
# [doc = " Print a condition of a let chain, `chain!(self, \"let Some(x) = y\")` will print"] # [doc = " `if let Some(x) = y` on the first call and `    && let Some(x) = y` thereafter"] macro_rules ! chain { ($ self : ident , $ ($ t : tt) *) => { if $ self . first . take () { println ! ("if {}" , format_args ! ($ ($ t) *)) ; } else { println ! ("    && {}" , format_args ! ($ ($ t) *)) ; } } }
};
}
