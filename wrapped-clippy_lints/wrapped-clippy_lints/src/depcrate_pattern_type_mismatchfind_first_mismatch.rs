// Generated macro for find_first_mismatch (function)
macro_rules! Depcrate_pattern_type_mismatchfind_first_mismatch {
() => {
// Module: crate::pattern_type_mismatch
// Provides: {"find_first_mismatch"}
// Dependencies: {}
fn find_first_mismatch (cx : & LateContext < '_ > , pat : & Pat < '_ >) -> Option < (Span , Mutability , Level) > { let mut result = None ; pat . walk (| p | { if result . is_some () { return false ; } if p . span . in_external_macro (cx . sess () . source_map ()) { return true ; } let adjust_pat = match p . kind { PatKind :: Or ([p , ..]) => p , _ => p , } ; if let Some (adjustments) = cx . typeck_results () . pat_adjustments () . get (adjust_pat . hir_id) && let [first , ..] = * * adjustments && let ty :: Ref (.. , mutability) = * first . source . kind () { let level = if p . hir_id == pat . hir_id { Level :: Top } else { Level :: Lower } ; result = Some ((p . span , mutability , level)) ; } result . is_none () }) ; result }
};
}
