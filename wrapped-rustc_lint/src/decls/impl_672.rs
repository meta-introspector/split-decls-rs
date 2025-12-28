macro_rules! deps {
    () => {
        LateContext!();
    };
}

macro_rules! impl_672 {
    () => {
        deps!();
        impl < 'tcx > LateLintPass < 'tcx > for NonPanicFmt { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < 'tcx >) { if let hir :: ExprKind :: Call (f , [arg]) = & expr . kind && let & ty :: FnDef (def_id , _) = cx . typeck_results () . expr_ty (f) . kind () { let f_diagnostic_name = cx . tcx . get_diagnostic_name (def_id) ; if cx . tcx . is_lang_item (def_id , LangItem :: BeginPanic) || cx . tcx . is_lang_item (def_id , LangItem :: Panic) || f_diagnostic_name == Some (sym :: panic_str_2015) { if let Some (id) = f . span . ctxt () . outer_expn_data () . macro_def_id { if matches ! (cx . tcx . get_diagnostic_name (id) , Some (sym :: core_panic_2015_macro | sym :: std_panic_2015_macro)) { check_panic (cx , f , arg) ; } } } else if f_diagnostic_name == Some (sym :: unreachable_display) { if let Some (id) = f . span . ctxt () . outer_expn_data () . macro_def_id && cx . tcx . is_diagnostic_item (sym :: unreachable_2015_macro , id) { check_panic (cx , f , match & arg . kind { hir :: ExprKind :: AddrOf (ast :: BorrowKind :: Ref , _ , arg) => arg , _ => bug ! ("call to unreachable_display without borrow") , } ,) ; } } } } }
    };
}

impl_672!()