// Generated macro for match_acceptable_type (function)
macro_rules! Depcrate_collection_is_never_readmatch_acceptable_type {
() => {
// Module: crate::collection_is_never_read
// Provides: {"match_acceptable_type"}
// Dependencies: {}
fn match_acceptable_type (cx : & LateContext < '_ > , local : & LetStmt < '_ >) -> bool { let ty = cx . typeck_results () . pat_ty (local . pat) ; matches ! (ty . opt_diag_name (cx) , Some (sym :: BTreeMap | sym :: BTreeSet | sym :: BinaryHeap | sym :: HashMap | sym :: HashSet | sym :: LinkedList | sym :: Option | sym :: Vec | sym :: VecDeque)) || ty . is_lang_item (cx , LangItem :: String) }
};
}
