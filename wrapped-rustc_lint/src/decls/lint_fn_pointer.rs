macro_rules! deps {
    () => {
        UnpredictableFunctionPointerComparisons!();
        LateContext!();
        ComparisonOp!();
        UnpredictableFunctionPointerComparisonsSuggestion!();
    };
}

macro_rules! lint_fn_pointer {
    () => {
        deps!();
        fn lint_fn_pointer < 'tcx > (cx : & LateContext < 'tcx > , e : & 'tcx hir :: Expr < 'tcx > , cmpop : ComparisonOp , l : & 'tcx hir :: Expr < 'tcx > , r : & 'tcx hir :: Expr < 'tcx > ,) { let peel_refs = | mut ty : Ty < 'tcx > | -> (Ty < 'tcx > , usize) { let mut refs = 0 ; while let ty :: Ref (_ , inner_ty , _) = ty . kind () { ty = * inner_ty ; refs += 1 ; } (ty , refs) } ; let l = l . peel_borrows () ; let r = r . peel_borrows () ; let Some (l_ty) = cx . typeck_results () . expr_ty_opt (l) else { return } ; let Some (r_ty) = cx . typeck_results () . expr_ty_opt (r) else { return } ; let (l_ty , l_ty_refs) = peel_refs (l_ty) ; let (r_ty , r_ty_refs) = peel_refs (r_ty) ; if l_ty . is_fn () && r_ty . is_fn () { } else if let ty :: Adt (l_def , l_args) = l_ty . kind () && let ty :: Adt (r_def , r_args) = r_ty . kind () && cx . tcx . is_lang_item (l_def . did () , LangItem :: Option) && cx . tcx . is_lang_item (r_def . did () , LangItem :: Option) && let Some (l_some_arg) = l_args . get (0) && let Some (r_some_arg) = r_args . get (0) && l_some_arg . expect_ty () . is_fn () && r_some_arg . expect_ty () . is_fn () { return cx . emit_span_lint (UNPREDICTABLE_FUNCTION_POINTER_COMPARISONS , e . span , UnpredictableFunctionPointerComparisons :: Warn ,) ; } else { return ; } let is_eq_ne = matches ! (cmpop , ComparisonOp :: BinOp (hir :: BinOpKind :: Eq | hir :: BinOpKind :: Ne)) ; if ! is_eq_ne { return cx . emit_span_lint (UNPREDICTABLE_FUNCTION_POINTER_COMPARISONS , e . span , UnpredictableFunctionPointerComparisons :: Warn ,) ; } let (Some (l_span) , Some (r_span)) = (l . span . find_ancestor_inside (e . span) , r . span . find_ancestor_inside (e . span)) else { return cx . emit_span_lint (UNPREDICTABLE_FUNCTION_POINTER_COMPARISONS , e . span , UnpredictableFunctionPointerComparisons :: Warn ,) ; } ; let ne = if cmpop == ComparisonOp :: BinOp (hir :: BinOpKind :: Ne) { "!" } else { "" } ; let deref_left = & * "*" . repeat (l_ty_refs) ; let deref_right = & * "*" . repeat (r_ty_refs) ; let left = e . span . shrink_to_lo () . until (l_span . shrink_to_lo ()) ; let middle = l_span . shrink_to_hi () . until (r_span . shrink_to_lo ()) ; let right = r_span . shrink_to_hi () . until (e . span . shrink_to_hi ()) ; let sugg = if ! r_ty . is_fn_ptr () { let fn_sig = r_ty . fn_sig (cx . tcx) ; UnpredictableFunctionPointerComparisonsSuggestion :: FnAddrEqWithCast { ne , fn_sig , deref_left , deref_right , left , middle , right , } } else { UnpredictableFunctionPointerComparisonsSuggestion :: FnAddrEq { ne , deref_left , deref_right , left , middle , right , } } ; cx . emit_span_lint (UNPREDICTABLE_FUNCTION_POINTER_COMPARISONS , e . span , UnpredictableFunctionPointerComparisons :: Suggestion { sugg } ,) ; }
    };
}

lint_fn_pointer!();