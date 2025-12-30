// Generated macro for find_pat_binding_and_is_innermost_parent_pat_struct (function)
macro_rules! Depcrate_matches_collapsible_matchfind_pat_binding_and_is_innermost_parent_pat_struct {
() => {
// Module: crate::matches::collapsible_match
// Provides: {"find_pat_binding_and_is_innermost_parent_pat_struct"}
// Dependencies: {}
fn find_pat_binding_and_is_innermost_parent_pat_struct (pat : & Pat < '_ > , hir_id : HirId) -> (Option < (Ident , Span) > , bool) { let mut binding = None ; let mut is_innermost_parent_pat_struct = false ; pat . walk_short (| p | match p . kind { PatKind :: Or (_) => false , PatKind :: Binding (_bm , _ , ident , _) => { let found = p . hir_id == hir_id ; if found { binding = Some ((ident , p . span)) ; } ! found } , _ => { is_innermost_parent_pat_struct = matches ! (p . kind , PatKind :: Struct (..)) ; true } , }) ; (binding , is_innermost_parent_pat_struct) }
};
}
