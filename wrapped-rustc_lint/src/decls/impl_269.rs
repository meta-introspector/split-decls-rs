macro_rules! deps {
    () => {
        QueryInstability!();
        LateContext!();
        QueryUntracked!();
    };
}

macro_rules! impl_269 {
    () => {
        deps!();
        impl < 'tcx > LateLintPass < 'tcx > for QueryStability { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) { if let Some ((callee_def_id , span , generic_args , _recv , _args)) = get_callee_span_generic_args_and_args (cx , expr) && let Ok (Some (instance)) = ty :: Instance :: try_resolve (cx . tcx , cx . typing_env () , callee_def_id , generic_args) { let def_id = instance . def_id () ; if cx . tcx . has_attr (def_id , sym :: rustc_lint_query_instability) { cx . emit_span_lint (POTENTIAL_QUERY_INSTABILITY , span , QueryInstability { query : cx . tcx . item_name (def_id) } ,) ; } else if has_unstable_into_iter_predicate (cx , callee_def_id , generic_args) { let call_span = span . with_hi (expr . span . hi ()) ; cx . emit_span_lint (POTENTIAL_QUERY_INSTABILITY , call_span , QueryInstability { query : sym :: into_iter } ,) ; } if cx . tcx . has_attr (def_id , sym :: rustc_lint_untracked_query_information) { cx . emit_span_lint (UNTRACKED_QUERY_INFORMATION , span , QueryUntracked { method : cx . tcx . item_name (def_id) } ,) ; } } } }
    };
}

impl_269!()