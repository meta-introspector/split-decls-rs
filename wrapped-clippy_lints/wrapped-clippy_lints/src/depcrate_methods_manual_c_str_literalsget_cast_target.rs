// Generated macro for get_cast_target (function)
macro_rules! Depcrate_methods_manual_c_str_literalsget_cast_target {
() => {
// Module: crate::methods::manual_c_str_literals
// Provides: {"get_cast_target"}
// Dependencies: {}
fn get_cast_target < 'tcx > (e : & 'tcx Expr < 'tcx >) -> Option < & 'tcx Expr < 'tcx > > { match & e . kind { ExprKind :: MethodCall (method , receiver , [] , _) if method . ident . name == sym :: cast => Some (receiver) , ExprKind :: Cast (expr , _) => Some (expr) , _ => None , } }
};
}
