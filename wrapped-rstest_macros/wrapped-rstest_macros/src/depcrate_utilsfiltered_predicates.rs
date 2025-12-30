// Generated macro for filtered_predicates (function)
macro_rules! Depcrate_utilsfiltered_predicates {
() => {
// Module: crate::utils
// Provides: {"filtered_predicates"}
// Dependencies: {}
fn filtered_predicates (mut wc : syn :: WhereClause , valids : & HashSet < Ident >) -> syn :: WhereClause { wc . predicates = wc . predicates . clone () . into_iter () . filter (| wp | { wp . maybe_ident () . map (| t | valids . contains (t)) . unwrap_or_default () }) . collect () ; wc }
};
}
