// Generated macro for is_body_const (function)
macro_rules! Depcrate_utilsis_body_const {
() => {
// Module: crate::utils
// Provides: {"is_body_const"}
// Dependencies: {}
pub fn is_body_const (sema : & Semantics < '_ , RootDatabase > , expr : & ast :: Expr) -> bool { let mut is_const = true ; preorder_expr (expr , & mut | ev | { let expr = match ev { WalkEvent :: Enter (_) if ! is_const => return true , WalkEvent :: Enter (expr) => expr , WalkEvent :: Leave (_) => return false , } ; match expr { ast :: Expr :: CallExpr (call) => { if let Some (ast :: Expr :: PathExpr (path_expr)) = call . expr () && let Some (PathResolution :: Def (ModuleDef :: Function (func))) = path_expr . path () . and_then (| path | sema . resolve_path (& path)) { is_const &= func . is_const (sema . db) ; } } ast :: Expr :: MethodCallExpr (call) => { is_const &= sema . resolve_method_call (& call) . map (| it | it . is_const (sema . db)) . unwrap_or (true) } ast :: Expr :: ForExpr (_) | ast :: Expr :: ReturnExpr (_) | ast :: Expr :: TryExpr (_) | ast :: Expr :: YieldExpr (_) | ast :: Expr :: AwaitExpr (_) => is_const = false , _ => () , } ! is_const }) ; is_const }
};
}
