macro_rules! deps {
    () => {
        LateContext!();
        BadOptAccessDiag!();
    };
}

macro_rules! impl_295 {
    () => {
        deps!();
        impl LateLintPass < '_ > for BadOptAccess { fn check_expr (& mut self , cx : & LateContext < '_ > , expr : & hir :: Expr < '_ >) { let hir :: ExprKind :: Field (base , target) = expr . kind else { return } ; let Some (adt_def) = cx . typeck_results () . expr_ty (base) . ty_adt_def () else { return } ; if ! cx . tcx . has_attr (adt_def . did () , sym :: rustc_lint_opt_ty) { return ; } for field in adt_def . all_fields () { if field . name == target . name && let Some (attr) = cx . tcx . get_attr (field . did , sym :: rustc_lint_opt_deny_field_access) && let Some (items) = attr . meta_item_list () && let Some (item) = items . first () && let Some (lit) = item . lit () && let ast :: LitKind :: Str (val , _) = lit . kind { cx . emit_span_lint (BAD_OPT_ACCESS , expr . span , BadOptAccessDiag { msg : val . as_str () } ,) ; } } } }
    };
}

impl_295!()