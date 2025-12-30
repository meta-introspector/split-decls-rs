// Generated macro for find_init (function)
macro_rules! Depcrate_shadowfind_init {
() => {
// Module: crate::shadow
// Provides: {"find_init"}
// Dependencies: {}
# [doc = " Finds the \"init\" expression for a pattern: `let <pat> = <init>;` (or `if let`) or"] # [doc = " `match <init> { .., <pat> => .., .. }`"] # [doc = ""] # [doc = " For closure arguments passed to a method call, returns the method call, and the `HirId` of the"] # [doc = " closure (which will later be skipped). This is for <https://github.com/rust-lang/rust-clippy/issues/10780>"] fn find_init < 'tcx > (cx : & LateContext < 'tcx > , hir_id : HirId) -> Option < (& 'tcx Expr < 'tcx > , Option < HirId >) > { for (hir_id , node) in cx . tcx . hir_parent_iter (hir_id) { let init = match node { Node :: Arm (_) | Node :: Pat (_) | Node :: PatField (_) | Node :: Param (_) => continue , Node :: Expr (expr) => match expr . kind { ExprKind :: Match (e , _ , _) | ExprKind :: Let (& LetExpr { init : e , .. }) => Some ((e , None)) , ExprKind :: Closure (_) => { if let Some ((_ , node)) = cx . tcx . hir_parent_iter (hir_id) . next () { match node { Node :: Expr (expr) => match expr . kind { ExprKind :: MethodCall (_ , _ , _ , _) | ExprKind :: Call (_ , _) => Some ((expr , Some (hir_id))) , _ => None , } , _ => None , } } else { None } } , _ => None , } , Node :: LetStmt (local) => local . init . map (| init | (init , None)) , _ => None , } ; return init ; } None }
};
}
