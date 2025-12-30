// Generated macro for is_res_used (function)
macro_rules! Depcrate_visitorsis_res_used {
() => {
// Module: crate::visitors
// Provides: {"is_res_used"}
// Dependencies: {}
# [doc = " Checks if the given resolved path is used in the given body."] pub fn is_res_used (cx : & LateContext < '_ > , res : Res , body : BodyId) -> bool { for_each_expr (cx , cx . tcx . hir_body (body) . value , | e | { if let ExprKind :: Path (p) = & e . kind && cx . qpath_res (p , e . hir_id) == res { return ControlFlow :: Break (()) ; } ControlFlow :: Continue (()) }) . is_some () }
};
}
