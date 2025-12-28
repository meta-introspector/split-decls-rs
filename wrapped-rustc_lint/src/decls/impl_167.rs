macro_rules! deps {
    () => {
        DropRefDiag!();
        ForgetRefDiag!();
        ForgetCopyDiag!();
        DropCopyDiag!();
        UndroppedManuallyDropsDiag!();
        UndroppedManuallyDropsSuggestion!();
        LateContext!();
        UseLetUnderscoreIgnoreSuggestion!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < 'tcx > LateLintPass < 'tcx > for DropForgetUseless { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) { if let ExprKind :: Call (path , [arg]) = expr . kind && let ExprKind :: Path (ref qpath) = path . kind && let Some (def_id) = cx . qpath_res (qpath , path . hir_id) . opt_def_id () && let Some (fn_name) = cx . tcx . get_diagnostic_name (def_id) { let arg_ty = cx . typeck_results () . expr_ty (arg) ; let is_copy = cx . type_is_copy_modulo_regions (arg_ty) ; let drop_is_single_call_in_arm = is_single_call_in_arm (cx , arg , expr) ; let let_underscore_ignore_sugg = | | { if let Some ((_ , node)) = cx . tcx . hir_parent_iter (expr . hir_id) . nth (0) && let Node :: Stmt (stmt) = node && let StmtKind :: Semi (e) = stmt . kind && e . hir_id == expr . hir_id && let Some (arg_span) = arg . span . find_ancestor_inside_same_ctxt (expr . span) { UseLetUnderscoreIgnoreSuggestion :: Suggestion { start_span : expr . span . shrink_to_lo () . until (arg_span) , end_span : arg_span . shrink_to_hi () . until (expr . span . shrink_to_hi ()) , } } else { UseLetUnderscoreIgnoreSuggestion :: Note } } ; match fn_name { sym :: mem_drop if arg_ty . is_ref () && ! drop_is_single_call_in_arm => { cx . emit_span_lint (DROPPING_REFERENCES , expr . span , DropRefDiag { arg_ty , label : arg . span , sugg : let_underscore_ignore_sugg () } ,) ; } sym :: mem_forget if arg_ty . is_ref () => { cx . emit_span_lint (FORGETTING_REFERENCES , expr . span , ForgetRefDiag { arg_ty , label : arg . span , sugg : let_underscore_ignore_sugg () , } ,) ; } sym :: mem_drop if is_copy && ! drop_is_single_call_in_arm => { cx . emit_span_lint (DROPPING_COPY_TYPES , expr . span , DropCopyDiag { arg_ty , label : arg . span , sugg : let_underscore_ignore_sugg () , } ,) ; } sym :: mem_forget if is_copy => { cx . emit_span_lint (FORGETTING_COPY_TYPES , expr . span , ForgetCopyDiag { arg_ty , label : arg . span , sugg : let_underscore_ignore_sugg () , } ,) ; } sym :: mem_drop if let ty :: Adt (adt , _) = arg_ty . kind () && adt . is_manually_drop () => { cx . emit_span_lint (UNDROPPED_MANUALLY_DROPS , expr . span , UndroppedManuallyDropsDiag { arg_ty , label : arg . span , suggestion : UndroppedManuallyDropsSuggestion { start_span : arg . span . shrink_to_lo () , end_span : arg . span . shrink_to_hi () , } , } ,) ; } _ => return , } ; } } }
    };
}

impl_167!()