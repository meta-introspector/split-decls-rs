// Generated macro for check_assert (function)
macro_rules! Depcrate_missing_asserts_for_indexingcheck_assert {
() => {
// Module: crate::missing_asserts_for_indexing
// Provides: {"check_assert"}
// Dependencies: {}
# [doc = " Checks if the expression is an `assert!` expression and adds it to `asserts`"] fn check_assert < 'hir > (cx : & LateContext < '_ > , expr : & 'hir Expr < 'hir > , map : & mut UnindexMap < u64 , Vec < IndexEntry < 'hir > > >) { if let Some ((comparison , asserted_len , slice , macro_call)) = assert_len_expr (cx , expr) { let hash = hash_expr (cx , slice) ; let indexes = map . entry (hash) . or_default () ; let entry = indexes . iter_mut () . find (| entry | eq_expr_value (cx , entry . slice () , slice)) ; if let Some (entry) = entry { if let IndexEntry :: IndexWithoutAssert { highest_index , is_first_highest , indexes , slice , } = entry && expr . span . lo () <= slice . span . lo () { * entry = IndexEntry :: AssertWithIndex { highest_index : * highest_index , indexes : mem :: take (indexes) , is_first_highest : * is_first_highest , slice , assert_span : expr . span . source_callsite () , comparison , asserted_len , macro_call , } ; } } else { indexes . push (IndexEntry :: StrayAssert { asserted_len , comparison , assert_span : expr . span . source_callsite () , slice , macro_call , }) ; } } }
};
}
