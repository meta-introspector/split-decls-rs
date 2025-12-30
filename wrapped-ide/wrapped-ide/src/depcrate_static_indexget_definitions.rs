// Generated macro for get_definitions (function)
macro_rules! Depcrate_static_indexget_definitions {
() => {
// Module: crate::static_index
// Provides: {"get_definitions"}
// Dependencies: {}
fn get_definitions (sema : & Semantics < '_ , RootDatabase > , token : SyntaxToken ,) -> Option < ArrayVec < Definition , 2 > > { for token in sema . descend_into_macros_exact (token) { let def = IdentClass :: classify_token (sema , & token) . map (IdentClass :: definitions_no_ops) ; if let Some (defs) = def && ! defs . is_empty () { return Some (defs) ; } } None }
};
}
