// Generated macro for into_iter_call (function)
macro_rules! Depcrate_useless_conversioninto_iter_call {
() => {
// Module: crate::useless_conversion
// Provides: {"into_iter_call"}
// Dependencies: {}
# [doc = " Extracts the receiver of a `.into_iter()` method call."] fn into_iter_call < 'hir > (cx : & LateContext < '_ > , expr : & 'hir Expr < 'hir >) -> Option < & 'hir Expr < 'hir > > { if let ExprKind :: MethodCall (name , recv , [] , _) = expr . kind && cx . ty_based_def (expr) . opt_parent (cx) . is_diag_item (cx , sym :: IntoIterator) && name . ident . name == sym :: into_iter { Some (recv) } else { None } }
};
}
