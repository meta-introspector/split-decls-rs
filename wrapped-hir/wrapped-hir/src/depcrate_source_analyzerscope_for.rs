// Generated macro for scope_for (function)
macro_rules! Depcrate_source_analyzerscope_for {
() => {
// Module: crate::source_analyzer
// Provides: {"scope_for"}
// Dependencies: {}
fn scope_for (db : & dyn HirDatabase , scopes : & ExprScopes , source_map : & BodySourceMap , node : InFile < & SyntaxNode > ,) -> Option < ScopeId > { node . ancestors_with_macros (db) . take_while (| it | { let kind = it . kind () ; ! ast :: Item :: can_cast (kind) || ast :: MacroCall :: can_cast (kind) || ast :: Use :: can_cast (kind) || ast :: AsmExpr :: can_cast (kind) }) . filter_map (| it | it . map (ast :: Expr :: cast) . transpose ()) . filter_map (| it | source_map . node_expr (it . as_ref ()) ? . as_expr ()) . find_map (| it | scopes . scope_for (it)) }
};
}
