macro_rules! deps {
    () => {
        LateContext!();
    };
}

macro_rules! impl_291 {
    () => {
        deps!();
        impl LateLintPass < '_ > for Diagnostics { fn check_expr < 'tcx > (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < 'tcx >) { let collect_args_tys_and_spans = | args : & [hir :: Expr < '_ >] , reserve_one_extra : bool | { let mut result = Vec :: with_capacity (args . len () + usize :: from (reserve_one_extra)) ; result . extend (args . iter () . map (| arg | (cx . typeck_results () . expr_ty (arg) , arg . span))) ; result } ; let Some ((def_id , span , fn_gen_args , recv , args)) = get_callee_span_generic_args_and_args (cx , expr) else { return ; } ; let mut arg_tys_and_spans = collect_args_tys_and_spans (args , recv . is_some ()) ; if let Some (recv) = recv { arg_tys_and_spans . insert (0 , (cx . tcx . types . self_param , recv . span)) ; } Self :: diagnostic_outside_of_impl (cx , span , expr . hir_id , def_id , fn_gen_args) ; Self :: untranslatable_diagnostic (cx , def_id , & arg_tys_and_spans) ; } }
    };
}

impl_291!()