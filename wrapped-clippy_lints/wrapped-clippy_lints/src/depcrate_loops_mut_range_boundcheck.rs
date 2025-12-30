// Generated macro for check (function)
macro_rules! Depcrate_loops_mut_range_boundcheck {
() => {
// Module: crate::loops::mut_range_bound
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , arg : & Expr < '_ > , body : & Expr < '_ >) { if let Some (higher :: Range { start : Some (start) , end : Some (end) , .. }) = higher :: Range :: hir (cx , arg) && let (mut_id_start , mut_id_end) = (check_for_mutability (cx , start) , check_for_mutability (cx , end)) && (mut_id_start . is_some () || mut_id_end . is_some ()) { let (span_low , span_high) = check_for_mutation (cx , body , mut_id_start , mut_id_end) ; mut_warn_with_span (cx , span_low) ; mut_warn_with_span (cx , span_high) ; } }
};
}
