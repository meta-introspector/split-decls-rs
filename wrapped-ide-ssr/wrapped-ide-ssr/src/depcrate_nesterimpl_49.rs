// Generated macro for impl_49 (impl)
macro_rules! Depcrate_nesterimpl_49 {
() => {
// Module: crate::nester
// Provides: {"impl_49"}
// Dependencies: {}
impl MatchCollector { # [doc = " Attempts to add `m` to matches. If it conflicts with an existing match, it is discarded. If"] # [doc = " it is entirely within the a placeholder of an existing match, then it is added as a child"] # [doc = " match of the existing match."] fn add_match (& mut self , m : Match , sema : & hir :: Semantics < '_ , ide_db :: RootDatabase >) { let matched_node = m . matched_node . clone () ; if let Some (existing) = self . matches_by_node . get_mut (& matched_node) { try_add_sub_match (m , existing , sema) ; return ; } for ancestor in sema . ancestors_with_macros (m . matched_node . clone ()) { if let Some (existing) = self . matches_by_node . get_mut (& ancestor) { try_add_sub_match (m , existing , sema) ; return ; } } self . matches_by_node . insert (matched_node , m) ; } }
};
}
