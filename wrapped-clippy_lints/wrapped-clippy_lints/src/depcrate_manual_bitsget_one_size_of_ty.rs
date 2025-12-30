// Generated macro for get_one_size_of_ty (function)
macro_rules! Depcrate_manual_bitsget_one_size_of_ty {
() => {
// Module: crate::manual_bits
// Provides: {"get_one_size_of_ty"}
// Dependencies: {}
fn get_one_size_of_ty < 'tcx > (cx : & LateContext < 'tcx > , expr1 : & 'tcx Expr < '_ > , expr2 : & 'tcx Expr < '_ > ,) -> Option < (Span , Ty < 'tcx > , & 'tcx Expr < 'tcx >) > { match (get_size_of_ty (cx , expr1) , get_size_of_ty (cx , expr2)) { (Some ((real_ty_span , resolved_ty)) , None) => Some ((real_ty_span , resolved_ty , expr2)) , (None , Some ((real_ty_span , resolved_ty))) => Some ((real_ty_span , resolved_ty , expr1)) , _ => None , } }
};
}
