macro_rules! deps {
    () => {
        SuspiciousDoubleRefDerefDiag!();
        SuspiciousDoubleRefCloneDiag!();
        NoopMethodCallDiag!();
        LateContext!();
    };
}

macro_rules! impl_714 {
    () => {
        deps!();
        impl < 'tcx > LateLintPass < 'tcx > for NoopMethodCall { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { let ExprKind :: MethodCall (call , receiver , _ , call_span) = & expr . kind else { return ; } ; if call_span . from_expansion () { return ; } let Some ((DefKind :: AssocFn , did)) = cx . typeck_results () . type_dependent_def (expr . hir_id) else { return ; } ; let Some (trait_id) = cx . tcx . trait_of_assoc (did) else { return } ; let Some (trait_) = cx . tcx . get_diagnostic_name (trait_id) else { return } ; if ! matches ! (trait_ , sym :: Borrow | sym :: Clone | sym :: Deref) { return ; } ; let args = cx . tcx . normalize_erasing_regions (cx . typing_env () , cx . typeck_results () . node_args (expr . hir_id)) ; let Ok (Some (i)) = ty :: Instance :: try_resolve (cx . tcx , cx . typing_env () , did , args) else { return ; } ; let Some (name) = cx . tcx . get_diagnostic_name (i . def_id ()) else { return } ; if ! matches ! (name , sym :: noop_method_borrow | sym :: noop_method_clone | sym :: noop_method_deref) { return ; } let receiver_ty = cx . typeck_results () . expr_ty (receiver) ; let expr_ty = cx . typeck_results () . expr_ty_adjusted (expr) ; let arg_adjustments = cx . typeck_results () . expr_adjustments (receiver) ; if arg_adjustments . iter () . any (| adj | matches ! (adj . kind , Adjust :: Deref (Some (_)))) { return ; } let expr_span = expr . span ; let span = expr_span . with_lo (receiver . span . hi ()) ; let orig_ty = expr_ty . peel_refs () ; if receiver_ty == expr_ty { let suggest_derive = match orig_ty . kind () { ty :: Adt (def , _) => Some (cx . tcx . def_span (def . did ()) . shrink_to_lo ()) , _ => None , } ; cx . emit_span_lint (NOOP_METHOD_CALL , span , NoopMethodCallDiag { method : call . ident , orig_ty , trait_ , label : span , suggest_derive , } ,) ; } else { match name { sym :: noop_method_borrow => return , sym :: noop_method_clone => cx . emit_span_lint (SUSPICIOUS_DOUBLE_REF_OP , span , SuspiciousDoubleRefCloneDiag { ty : expr_ty } ,) , sym :: noop_method_deref => cx . emit_span_lint (SUSPICIOUS_DOUBLE_REF_OP , span , SuspiciousDoubleRefDerefDiag { ty : expr_ty } ,) , _ => return , } } } }
    };
}

impl_714!()