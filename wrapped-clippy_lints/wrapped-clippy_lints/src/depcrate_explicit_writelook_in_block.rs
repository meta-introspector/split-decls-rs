// Generated macro for look_in_block (function)
macro_rules! Depcrate_explicit_writelook_in_block {
() => {
// Module: crate::explicit_write
// Provides: {"look_in_block"}
// Dependencies: {}
# [doc = " If `kind` is a block that looks like `{ let result = $expr; result }` then"] # [doc = " returns $expr. Otherwise returns `kind`."] fn look_in_block < 'tcx , 'hir > (cx : & LateContext < 'tcx > , kind : & 'tcx ExprKind < 'hir >) -> & 'tcx ExprKind < 'hir > { if let ExprKind :: Block (block , _label @ None) = kind && let Block { stmts : [Stmt { kind : StmtKind :: Let (local) , .. }] , expr : Some (expr_end_of_block) , rules : BlockCheckMode :: DefaultBlock , .. } = block && let ExprKind :: Path (QPath :: Resolved (None , expr_path)) = expr_end_of_block . kind && let Res :: Local (expr_res) = expr_path . res && let Node :: Pat (res_pat) = cx . tcx . hir_node (expr_res) && let PatKind :: Binding (BindingMode :: NONE , local_hir_id , _ident , None) = local . pat . kind && res_pat . hir_id == local_hir_id && let Some (init) = local . init { return & init . kind ; } kind }
};
}
