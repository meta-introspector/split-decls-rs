macro_rules! deps {
    () => {
        InvalidNanComparisonsSuggestion!();
        LateContext!();
        InvalidNanComparisons!();
    };
}

macro_rules! lint_nan {
    () => {
        deps!();
        fn lint_nan < 'tcx > (cx : & LateContext < 'tcx > , e : & 'tcx hir :: Expr < 'tcx > , binop : hir :: BinOpKind , l : & 'tcx hir :: Expr < 'tcx > , r : & 'tcx hir :: Expr < 'tcx > ,) { fn is_nan (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ >) -> bool { let expr = expr . peel_blocks () . peel_borrows () ; match expr . kind { ExprKind :: Path (qpath) => { let Some (def_id) = cx . typeck_results () . qpath_res (& qpath , expr . hir_id) . opt_def_id () else { return false ; } ; matches ! (cx . tcx . get_diagnostic_name (def_id) , Some (sym :: f16_nan | sym :: f32_nan | sym :: f64_nan | sym :: f128_nan)) } _ => false , } } fn eq_ne (e : & hir :: Expr < '_ > , l : & hir :: Expr < '_ > , r : & hir :: Expr < '_ > , f : impl FnOnce (Span , Span) -> InvalidNanComparisonsSuggestion ,) -> InvalidNanComparisons { let suggestion = if let Some (l_span) = l . span . find_ancestor_inside (e . span) && let Some (r_span) = r . span . find_ancestor_inside (e . span) { f (l_span , r_span) } else { InvalidNanComparisonsSuggestion :: Spanless } ; InvalidNanComparisons :: EqNe { suggestion } } let lint = match binop { hir :: BinOpKind :: Eq | hir :: BinOpKind :: Ne if is_nan (cx , l) => { eq_ne (e , l , r , | l_span , r_span | InvalidNanComparisonsSuggestion :: Spanful { nan_plus_binop : l_span . until (r_span) , float : r_span . shrink_to_hi () , neg : (binop == hir :: BinOpKind :: Ne) . then (| | r_span . shrink_to_lo ()) , }) } hir :: BinOpKind :: Eq | hir :: BinOpKind :: Ne if is_nan (cx , r) => { eq_ne (e , l , r , | l_span , r_span | InvalidNanComparisonsSuggestion :: Spanful { nan_plus_binop : l_span . shrink_to_hi () . to (r_span) , float : l_span . shrink_to_hi () , neg : (binop == hir :: BinOpKind :: Ne) . then (| | l_span . shrink_to_lo ()) , }) } hir :: BinOpKind :: Lt | hir :: BinOpKind :: Le | hir :: BinOpKind :: Gt | hir :: BinOpKind :: Ge if is_nan (cx , l) || is_nan (cx , r) => { InvalidNanComparisons :: LtLeGtGe } _ => return , } ; cx . emit_span_lint (INVALID_NAN_COMPARISONS , e . span , lint) ; }
    };
}

lint_nan!();