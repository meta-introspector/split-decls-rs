macro_rules! deps {
    () => {
        LateContext!();
    };
}

macro_rules! extract_iterator_next_call {
    () => {
        deps!();
        fn extract_iterator_next_call < 'tcx > (cx : & LateContext < '_ > , expr : & Expr < 'tcx > ,) -> Option < & 'tcx Expr < 'tcx > > { if let hir :: ExprKind :: MethodCall (_ , recv , _ , _) = expr . kind && cx . typeck_results () . type_dependent_def_id (expr . hir_id) . is_some_and (| def_id | cx . tcx . is_lang_item (def_id , LangItem :: IteratorNext)) { Some (recv) } else { None } }
    };
}

extract_iterator_next_call!()