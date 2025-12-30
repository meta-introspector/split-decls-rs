// Generated macro for check (function)
macro_rules! Depcrate_methods_drain_collectcheck {
() => {
// Module: crate::methods::drain_collect
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , args : & [Expr < '_ >] , expr : & Expr < '_ > , recv : & Expr < '_ >) { let expr_ty = cx . typeck_results () . expr_ty (expr) ; let recv_ty = cx . typeck_results () . expr_ty (recv) ; let recv_ty_no_refs = recv_ty . peel_refs () ; if let ExprKind :: Path (QPath :: Resolved (_ , recv_path)) = recv . kind && let Some (typename) = check_vec (cx , args , expr_ty , recv_ty_no_refs , recv_path) . then_some ("Vec") . or_else (| | check_string (cx , args , expr_ty , recv_ty_no_refs , recv_path) . then_some ("String")) . or_else (| | check_collections (cx , expr_ty , recv_ty_no_refs)) && let Some (exec_context) = std_or_core (cx) { let recv = snippet (cx , recv . span , "<expr>") ; let sugg = if let ty :: Ref (..) = recv_ty . kind () { format ! ("{exec_context}::mem::take({recv})") } else { format ! ("{exec_context}::mem::take(&mut {recv})") } ; span_lint_and_sugg (cx , DRAIN_COLLECT , expr . span , format ! ("you seem to be trying to move all elements into a new `{typename}`") , "consider using `mem::take`" , sugg , Applicability :: MachineApplicable ,) ; } }
};
}
