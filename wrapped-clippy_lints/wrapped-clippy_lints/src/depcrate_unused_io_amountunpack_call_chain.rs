// Generated macro for unpack_call_chain (function)
macro_rules! Depcrate_unused_io_amountunpack_call_chain {
() => {
// Module: crate::unused_io_amount
// Provides: {"unpack_call_chain"}
// Dependencies: {}
fn unpack_call_chain < 'a > (mut expr : & 'a hir :: Expr < 'a >) -> & 'a hir :: Expr < 'a > { while let ExprKind :: MethodCall (path , receiver , ..) = expr . kind { if matches ! (path . ident . name , sym :: unwrap | sym :: expect | sym :: unwrap_or | sym :: unwrap_or_else | sym :: ok | sym :: is_ok | sym :: is_err | sym :: or_else | sym :: or) { expr = receiver ; } else { break ; } } expr }
};
}
