// Generated macro for extract_init_or_reserve_target (function)
macro_rules! Depcrate_uninit_vecextract_init_or_reserve_target {
() => {
// Module: crate::uninit_vec
// Provides: {"extract_init_or_reserve_target"}
// Dependencies: {}
# [doc = " Finds the target location where the result of `Vec` initialization is stored"] # [doc = " or `self` expression for `Vec::reserve()`."] fn extract_init_or_reserve_target < 'tcx > (cx : & LateContext < 'tcx > , stmt : & 'tcx Stmt < 'tcx >) -> Option < TargetVec < 'tcx > > { match stmt . kind { StmtKind :: Let (local) => { if let Some (init_expr) = local . init && let PatKind :: Binding (_ , hir_id , _ , None) = local . pat . kind && let Some (init_kind) = get_vec_init_kind (cx , init_expr) { return Some (TargetVec { location : VecLocation :: Local (hir_id) , init_kind : Some (init_kind) , }) ; } } , StmtKind :: Expr (expr) | StmtKind :: Semi (expr) => match expr . kind { ExprKind :: Assign (lhs , rhs , _span) => { if let Some (init_kind) = get_vec_init_kind (cx , rhs) { return Some (TargetVec { location : VecLocation :: Expr (lhs) , init_kind : Some (init_kind) , }) ; } } , ExprKind :: MethodCall (path , self_expr , [_] , _) if is_reserve (cx , path , self_expr) => { return Some (TargetVec { location : VecLocation :: Expr (self_expr) , init_kind : None , }) ; } , _ => () , } , StmtKind :: Item (_) => () , } None }
};
}
