// Generated macro for match_types (function)
macro_rules! Depcrate_rendermatch_types {
() => {
// Module: crate::render
// Provides: {"match_types"}
// Dependencies: {}
fn match_types (ctx : & CompletionContext < '_ > , ty1 : & hir :: Type < '_ > , ty2 : & hir :: Type < '_ > ,) -> Option < CompletionRelevanceTypeMatch > { if ty1 == ty2 { Some (CompletionRelevanceTypeMatch :: Exact) } else if ty1 . could_unify_with (ctx . db , ty2) { Some (CompletionRelevanceTypeMatch :: CouldUnify) } else { None } }
};
}
