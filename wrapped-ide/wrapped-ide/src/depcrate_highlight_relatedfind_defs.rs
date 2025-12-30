// Generated macro for find_defs (function)
macro_rules! Depcrate_highlight_relatedfind_defs {
() => {
// Module: crate::highlight_related
// Provides: {"find_defs"}
// Dependencies: {}
fn find_defs (sema : & Semantics < '_ , RootDatabase > , token : SyntaxToken) -> FxHashSet < Definition > { sema . descend_into_macros_exact (token) . into_iter () . filter_map (| token | IdentClass :: classify_token (sema , & token)) . flat_map (IdentClass :: definitions_no_ops) . collect () }
};
}
