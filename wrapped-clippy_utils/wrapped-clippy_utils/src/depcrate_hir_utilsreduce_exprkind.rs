// Generated macro for reduce_exprkind (function)
macro_rules! Depcrate_hir_utilsreduce_exprkind {
() => {
// Module: crate::hir_utils
// Provides: {"reduce_exprkind"}
// Dependencies: {}
# [doc = " Some simple reductions like `{ return }` => `return`"] fn reduce_exprkind < 'hir > (cx : & LateContext < '_ > , kind : & 'hir ExprKind < 'hir >) -> & 'hir ExprKind < 'hir > { if let ExprKind :: Block (block , _) = kind { match (block . stmts , block . expr) { ([] , None) if block . span . is_empty () => & ExprKind :: Tup (& []) , ([] , None) if block . span . check_source_text (cx , | src | { tokenize (src , FrontmatterAllowed :: No) . map (| t | t . kind) . filter (| t | { ! matches ! (t , TokenKind :: LineComment { .. } | TokenKind :: BlockComment { .. } | TokenKind :: Whitespace) }) . eq ([TokenKind :: OpenBrace , TokenKind :: CloseBrace] . iter () . copied ()) }) => { & ExprKind :: Tup (& []) } , ([] , Some (expr)) => match expr . kind { ExprKind :: Ret (..) => & expr . kind , _ => kind , } , ([stmt] , None) => match stmt . kind { StmtKind :: Expr (expr) | StmtKind :: Semi (expr) => match expr . kind { ExprKind :: Ret (..) => & expr . kind , _ => kind , } , _ => kind , } , _ => kind , } } else { kind } }
};
}
