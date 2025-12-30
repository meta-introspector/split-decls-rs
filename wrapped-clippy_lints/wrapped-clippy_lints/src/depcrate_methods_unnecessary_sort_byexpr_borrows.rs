// Generated macro for expr_borrows (function)
macro_rules! Depcrate_methods_unnecessary_sort_byexpr_borrows {
() => {
// Module: crate::methods::unnecessary_sort_by
// Provides: {"expr_borrows"}
// Dependencies: {}
fn expr_borrows (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { let ty = cx . typeck_results () . expr_ty (expr) ; matches ! (ty . kind () , ty :: Ref (..)) || ty . walk () . any (| arg | matches ! (arg . kind () , GenericArgKind :: Lifetime (_))) }
};
}
