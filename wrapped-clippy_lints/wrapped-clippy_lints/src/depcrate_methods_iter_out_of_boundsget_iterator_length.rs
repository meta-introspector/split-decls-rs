// Generated macro for get_iterator_length (function)
macro_rules! Depcrate_methods_iter_out_of_boundsget_iterator_length {
() => {
// Module: crate::methods::iter_out_of_bounds
// Provides: {"get_iterator_length"}
// Dependencies: {}
# [doc = " Attempts to extract the length out of an iterator expression."] fn get_iterator_length < 'tcx > (cx : & LateContext < 'tcx > , iter : & 'tcx Expr < 'tcx >) -> Option < u128 > { let ty :: Adt (adt , substs) = cx . typeck_results () . expr_ty (iter) . kind () else { return None ; } ; match cx . tcx . get_diagnostic_name (adt . did ()) { Some (sym :: ArrayIntoIter) => { substs . const_at (1) . try_to_target_usize (cx . tcx) . map (u128 :: from) } , Some (sym :: SliceIter) if let ExprKind :: MethodCall (_ , recv , ..) = iter . kind => { if let ty :: Array (_ , len) = cx . typeck_results () . expr_ty (recv) . peel_refs () . kind () { len . try_to_target_usize (cx . tcx) . map (u128 :: from) } else if let Some (args) = VecArgs :: hir (cx , expr_or_init (cx , recv)) { match args { VecArgs :: Vec (vec) => vec . len () . try_into () . ok () , VecArgs :: Repeat (_ , len) => expr_as_u128 (cx , len) , } } else { None } } , Some (sym :: IterEmpty) => Some (0) , Some (sym :: IterOnce) => Some (1) , _ => None , } }
};
}
