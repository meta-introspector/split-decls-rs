// Generated macro for check_index (function)
macro_rules! Depcrate_missing_asserts_for_indexingcheck_index {
() => {
// Module: crate::missing_asserts_for_indexing
// Provides: {"check_index"}
// Dependencies: {}
# [doc = " Checks if the expression is an index into a slice and adds it to `indexes`"] fn check_index < 'hir > (cx : & LateContext < '_ > , expr : & 'hir Expr < 'hir > , map : & mut UnindexMap < u64 , Vec < IndexEntry < 'hir > > >) { if let ExprKind :: Index (slice , index_lit , _) = expr . kind && cx . typeck_results () . expr_ty_adjusted (slice) . peel_refs () . is_slice () && let Some (index) = upper_index_expr (cx , index_lit) { let hash = hash_expr (cx , slice) ; let indexes = map . entry (hash) . or_default () ; let entry = indexes . iter_mut () . find (| entry | eq_expr_value (cx , entry . slice () , slice)) ; if let Some (entry) = entry { match entry { IndexEntry :: StrayAssert { asserted_len , comparison , assert_span , slice , macro_call , } => { if slice . span . lo () > assert_span . lo () { * entry = IndexEntry :: AssertWithIndex { highest_index : index , is_first_highest : true , asserted_len : * asserted_len , assert_span : * assert_span , slice , indexes : vec ! [expr . span] , comparison : * comparison , macro_call : * macro_call , } ; } } , IndexEntry :: IndexWithoutAssert { highest_index , indexes , is_first_highest , .. } | IndexEntry :: AssertWithIndex { highest_index , indexes , is_first_highest , .. } => { indexes . push (expr . span) ; if * is_first_highest { (* is_first_highest) = * highest_index >= index ; } * highest_index = (* highest_index) . max (index) ; } , } } else { indexes . push (IndexEntry :: IndexWithoutAssert { highest_index : index , is_first_highest : true , indexes : vec ! [expr . span] , slice , }) ; } } }
};
}
