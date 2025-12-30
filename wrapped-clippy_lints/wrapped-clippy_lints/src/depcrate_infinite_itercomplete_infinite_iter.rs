// Generated macro for complete_infinite_iter (function)
macro_rules! Depcrate_infinite_itercomplete_infinite_iter {
() => {
// Module: crate::infinite_iter
// Provides: {"complete_infinite_iter"}
// Dependencies: {}
fn complete_infinite_iter (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> Finiteness { match expr . kind { ExprKind :: MethodCall (method , receiver , args , _) => { let method_str = method . ident . name ; for & (name , len) in & COMPLETING_METHODS { if method_str == name && args . len () == len { return is_infinite (cx , receiver) ; } } for & (name , len) in & POSSIBLY_COMPLETING_METHODS { if method_str == name && args . len () == len { return MaybeInfinite . and (is_infinite (cx , receiver)) ; } } if method . ident . name == sym :: last && args . is_empty () { let not_double_ended = cx . tcx . get_diagnostic_item (sym :: DoubleEndedIterator) . is_some_and (| id | ! implements_trait (cx , cx . typeck_results () . expr_ty (receiver) , id , & [])) ; if not_double_ended { return is_infinite (cx , receiver) ; } } else if method . ident . name == sym :: collect { let ty = cx . typeck_results () . expr_ty (expr) ; if matches ! (ty . opt_diag_name (cx) , Some (sym :: BinaryHeap | sym :: BTreeMap | sym :: BTreeSet | sym :: HashMap | sym :: HashSet | sym :: LinkedList | sym :: Vec | sym :: VecDeque ,)) { return is_infinite (cx , receiver) ; } } } , ExprKind :: Binary (op , l , r) => { if op . node . is_comparison () { return is_infinite (cx , l) . and (is_infinite (cx , r)) . and (MaybeInfinite) ; } } , _ => () , } Finite }
};
}
