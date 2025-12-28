macro_rules! deps {
    () => {
        LateContext!();
    };
}

macro_rules! get_callee_span_generic_args_and_args {
    () => {
        deps!();
        # [doc = " Checks whether an expression is a function or method call and, if so, returns its `DefId`,"] # [doc = " `Span`, `GenericArgs`, and arguments. This is a slight augmentation of a similarly named Clippy"] # [doc = " function, `get_callee_generic_args_and_args`."] fn get_callee_span_generic_args_and_args < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > ,) -> Option < (DefId , Span , GenericArgsRef < 'tcx > , Option < & 'tcx Expr < 'tcx > > , & 'tcx [Expr < 'tcx >]) > { if let ExprKind :: Call (callee , args) = expr . kind && let callee_ty = cx . typeck_results () . expr_ty (callee) && let ty :: FnDef (callee_def_id , generic_args) = callee_ty . kind () { return Some ((* callee_def_id , callee . span , generic_args , None , args)) ; } if let ExprKind :: MethodCall (segment , recv , args , _) = expr . kind && let Some (method_def_id) = cx . typeck_results () . type_dependent_def_id (expr . hir_id) { let generic_args = cx . typeck_results () . node_args (expr . hir_id) ; return Some ((method_def_id , segment . ident . span , generic_args , Some (recv) , args)) ; } None }
    };
}

get_callee_span_generic_args_and_args!();