macro_rules! deps {
    () => {
        MappingToUnit!();
        LateContext!();
    };
}

macro_rules! impl_656 {
    () => {
        deps!();
        impl < 'tcx > LateLintPass < 'tcx > for MapUnitFn { fn check_stmt (& mut self , cx : & LateContext < 'tcx > , stmt : & Stmt < '_ >) { let StmtKind :: Semi (expr) = stmt . kind else { return ; } ; let ExprKind :: MethodCall (path , receiver , [arg] , span) = expr . kind else { return ; } ; if path . ident . name != sym :: map || stmt . span . from_expansion () || receiver . span . from_expansion () || arg . span . from_expansion () || ! is_impl_slice (cx , receiver) || ! cx . typeck_results () . type_dependent_def_id (expr . hir_id) . is_some_and (| id | cx . tcx . is_diagnostic_item (sym :: IteratorMap , id)) { return ; } let (id , sig) = match * cx . typeck_results () . expr_ty (arg) . kind () { ty :: Closure (id , subs) => (id , subs . as_closure () . sig ()) , ty :: FnDef (id , _) => (id , cx . tcx . fn_sig (id) . skip_binder ()) , _ => return , } ; let ret_ty = sig . output () . skip_binder () ; if ! (ret_ty . is_unit () || ret_ty . is_never ()) { return ; } cx . emit_span_lint (MAP_UNIT_FN , span , MappingToUnit { function_label : cx . tcx . span_of_impl (id) . unwrap_or (arg . span) , argument_label : arg . span , map_label : span , suggestion : path . ident . span , } ,) ; } }
    };
}

impl_656!();