// Generated macro for closure_body_returns_empty_to_string (function)
macro_rules! Depcrate_methods_or_fun_callclosure_body_returns_empty_to_string {
() => {
// Module: crate::methods::or_fun_call
// Provides: {"closure_body_returns_empty_to_string"}
// Dependencies: {}
fn closure_body_returns_empty_to_string (cx : & LateContext < '_ > , e : & hir :: Expr < '_ >) -> bool { if let hir :: ExprKind :: Closure (& hir :: Closure { body , .. }) = e . kind { let body = cx . tcx . hir_body (body) ; if body . params . is_empty () && let hir :: Expr { kind , .. } = & body . value && let hir :: ExprKind :: MethodCall (hir :: PathSegment { ident , .. } , self_arg , [] , _) = kind && ident . name == sym :: to_string && let hir :: Expr { kind , .. } = self_arg && let hir :: ExprKind :: Lit (lit) = kind && let ast :: LitKind :: Str (rustc_span :: sym :: empty , _) = lit . node { return true ; } } false }
};
}
