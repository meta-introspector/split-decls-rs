// Generated macro for reduce_unit_expression (function)
macro_rules! Depcrate_map_unit_fnreduce_unit_expression {
() => {
// Module: crate::map_unit_fn
// Provides: {"reduce_unit_expression"}
// Dependencies: {}
# [doc = " The expression inside a closure may or may not have surrounding braces and"] # [doc = " semicolons, which causes problems when generating a suggestion. Given an"] # [doc = " expression that evaluates to '()' or '!', recursively remove useless braces"] # [doc = " and semi-colons until is suitable for including in the suggestion template."] # [doc = " The `bool` is `true` when the resulting `span` needs to be enclosed in an"] # [doc = " `unsafe` block."] fn reduce_unit_expression (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ >) -> Option < (Span , bool) > { if ! is_unit_expression (cx , expr) { return None ; } match expr . kind { hir :: ExprKind :: Call (_ , _) | hir :: ExprKind :: MethodCall (..) => { Some ((expr . span , false)) } , hir :: ExprKind :: Block (block , _) => { let is_unsafe = matches ! (block . rules , hir :: BlockCheckMode :: UnsafeBlock (_)) ; match (block . stmts , block . expr . as_ref ()) { ([] , Some (inner_expr)) => { reduce_unit_expression (cx , inner_expr) . map (| (span , inner_is_unsafe) | (span , inner_is_unsafe || is_unsafe)) } , ([inner_stmt] , None) => { match inner_stmt . kind { hir :: StmtKind :: Let (local) => Some ((local . span , is_unsafe)) , hir :: StmtKind :: Expr (e) => Some ((e . span , is_unsafe)) , hir :: StmtKind :: Semi (..) => Some ((inner_stmt . span , is_unsafe)) , hir :: StmtKind :: Item (..) => None , } } , _ => { None } , } } , _ => None , } }
};
}
