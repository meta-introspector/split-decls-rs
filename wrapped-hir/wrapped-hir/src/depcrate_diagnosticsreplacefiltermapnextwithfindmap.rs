// Generated macro for ReplaceFilterMapNextWithFindMap (struct)
macro_rules! Depcrate_diagnosticsReplaceFilterMapNextWithFindMap {
() => {
// Module: crate::diagnostics
// Provides: {"ReplaceFilterMapNextWithFindMap"}
// Dependencies: {}
# [derive (Debug)] pub struct ReplaceFilterMapNextWithFindMap { pub file : HirFileId , # [doc = " This expression is the whole method chain up to and including `.filter_map(..).next()`."] pub next_expr : AstPtr < ast :: Expr > , }
};
}
