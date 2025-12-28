macro_rules! deps {
    () => {
        CheckTransmutes!();
        LateContext!();
    };
}

macro_rules! impl_790 {
    () => {
        deps!();
        impl < 'tcx > LateLintPass < 'tcx > for CheckTransmutes { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < 'tcx >) { let hir :: ExprKind :: Call (callee , [arg]) = expr . kind else { return ; } ; let hir :: ExprKind :: Path (qpath) = callee . kind else { return ; } ; let Res :: Def (DefKind :: Fn , def_id) = cx . qpath_res (& qpath , callee . hir_id) else { return ; } ; if ! cx . tcx . is_intrinsic (def_id , sym :: transmute) { return ; } ; let body_owner_def_id = cx . tcx . hir_enclosing_body_owner (expr . hir_id) ; let const_context = cx . tcx . hir_body_const_context (body_owner_def_id) ; let args = cx . typeck_results () . node_args (callee . hir_id) ; let src = args . type_at (0) ; let dst = args . type_at (1) ; check_ptr_transmute_in_const (cx , expr , body_owner_def_id , const_context , src , dst) ; check_unnecessary_transmute (cx , expr , callee , arg , const_context , src , dst) ; check_int_to_ptr_transmute (cx , expr , arg , src , dst) ; } }
    };
}

impl_790!()