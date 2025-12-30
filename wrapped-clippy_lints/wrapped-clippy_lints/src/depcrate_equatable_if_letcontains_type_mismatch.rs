// Generated macro for contains_type_mismatch (function)
macro_rules! Depcrate_equatable_if_letcontains_type_mismatch {
() => {
// Module: crate::equatable_if_let
// Provides: {"contains_type_mismatch"}
// Dependencies: {}
# [doc = " Check if the pattern has any type mismatch that would prevent it from being used in an equality"] # [doc = " check. This can happen if the expr has a reference type and the corresponding pattern is a"] # [doc = " literal."] fn contains_type_mismatch (cx : & LateContext < '_ > , pat : & Pat < '_ >) -> bool { let mut result = false ; pat . walk (| p | { if result { return false ; } if p . span . in_external_macro (cx . sess () . source_map ()) { return true ; } let adjust_pat = match p . kind { PatKind :: Or ([p , ..]) => p , _ => p , } ; if let Some (adjustments) = cx . typeck_results () . pat_adjustments () . get (adjust_pat . hir_id) && adjustments . first () . is_some_and (| first | first . source . is_ref ()) { result = true ; return false ; } true }) ; result }
};
}
