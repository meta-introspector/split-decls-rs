// Generated macro for check_io_mode (function)
macro_rules! Depcrate_unused_io_amountcheck_io_mode {
() => {
// Module: crate::unused_io_amount
// Provides: {"check_io_mode"}
// Dependencies: {}
# [doc = " Check whether the current expr is a function call for an IO operation"] fn check_io_mode (cx : & LateContext < '_ > , call : & hir :: Expr < '_ >) -> Option < IoOp > { let ExprKind :: MethodCall (path , ..) = call . kind else { return None ; } ; let vectorized = match path . ident . as_str () { "write_vectored" | "read_vectored" => true , "write" | "read" => false , _ => { return None ; } , } ; if let Some (method_def_id) = cx . typeck_results () . type_dependent_def_id (call . hir_id) && let Some (trait_def_id) = cx . tcx . trait_of_assoc (method_def_id) { if let Some (diag_name) = cx . tcx . get_diagnostic_name (trait_def_id) { match diag_name { sym :: IoRead => Some (IoOp :: SyncRead (vectorized)) , sym :: IoWrite => Some (IoOp :: SyncWrite (vectorized)) , _ => None , } } else if paths :: FUTURES_IO_ASYNCREADEXT . matches (cx , trait_def_id) || paths :: TOKIO_IO_ASYNCREADEXT . matches (cx , trait_def_id) { Some (IoOp :: AsyncRead (vectorized)) } else if paths :: TOKIO_IO_ASYNCWRITEEXT . matches (cx , trait_def_id) || paths :: FUTURES_IO_ASYNCWRITEEXT . matches (cx , trait_def_id) { Some (IoOp :: AsyncWrite (vectorized)) } else { None } } else { None } }
};
}
