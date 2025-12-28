macro_rules! deps {
    () => {
        UselessPtrNullChecksDiag!();
        LateContext!();
    };
}

macro_rules! useless_check {
    () => {
        deps!();
        # [doc = " This function checks if the expression is from a series of consecutive casts,"] # [doc = " ie. `(my_fn as *const _ as *mut _).cast_mut()` and whether the original expression is either"] # [doc = " a fn ptr, a reference, or a function call whose definition is"] # [doc = " annotated with `#![rustc_never_returns_null_ptr]`."] # [doc = " If this situation is present, the function returns the appropriate diagnostic."] fn useless_check < 'a , 'tcx : 'a > (cx : & 'a LateContext < 'tcx > , mut e : & 'a Expr < 'a > ,) -> Option < UselessPtrNullChecksDiag < 'tcx > > { let mut had_at_least_one_cast = false ; loop { e = e . peel_blocks () ; if let ExprKind :: MethodCall (_ , _expr , [] , _) = e . kind && let Some (def_id) = cx . typeck_results () . type_dependent_def_id (e . hir_id) && cx . tcx . has_attr (def_id , sym :: rustc_never_returns_null_ptr) && let Some (fn_name) = cx . tcx . opt_item_ident (def_id) { return Some (UselessPtrNullChecksDiag :: FnRet { fn_name }) ; } else if let ExprKind :: Call (path , _args) = e . kind && let ExprKind :: Path (ref qpath) = path . kind && let Some (def_id) = cx . qpath_res (qpath , path . hir_id) . opt_def_id () && cx . tcx . has_attr (def_id , sym :: rustc_never_returns_null_ptr) && let Some (fn_name) = cx . tcx . opt_item_ident (def_id) { return Some (UselessPtrNullChecksDiag :: FnRet { fn_name }) ; } e = if let ExprKind :: Cast (expr , t) = e . kind && let TyKind :: Ptr (_) = t . kind { had_at_least_one_cast = true ; expr } else if let ExprKind :: MethodCall (_ , expr , [] , _) = e . kind && let Some (def_id) = cx . typeck_results () . type_dependent_def_id (e . hir_id) && matches ! (cx . tcx . get_diagnostic_name (def_id) , Some (sym :: ptr_cast | sym :: ptr_cast_mut)) { had_at_least_one_cast = true ; expr } else if had_at_least_one_cast { let orig_ty = cx . typeck_results () . expr_ty (e) ; return if orig_ty . is_fn () { Some (UselessPtrNullChecksDiag :: FnPtr { orig_ty , label : e . span }) } else if orig_ty . is_ref () { Some (UselessPtrNullChecksDiag :: Ref { orig_ty , label : e . span }) } else { None } ; } else { return None ; } ; } }
    };
}

useless_check!()