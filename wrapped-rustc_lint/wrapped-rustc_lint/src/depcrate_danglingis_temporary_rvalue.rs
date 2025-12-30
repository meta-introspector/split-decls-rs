// Generated macro for is_temporary_rvalue (function)
macro_rules! Depcrate_danglingis_temporary_rvalue {
() => {
// Module: crate::dangling
// Provides: {"is_temporary_rvalue"}
// Dependencies: {}
fn is_temporary_rvalue (expr : & Expr < '_ >) -> bool { match expr . kind { ExprKind :: ConstBlock (..) | ExprKind :: Repeat (..) | ExprKind :: Lit (..) => false , ExprKind :: Path (..) => false , ExprKind :: Call (..) | ExprKind :: MethodCall (..) | ExprKind :: Use (..) | ExprKind :: Binary (..) => true , ExprKind :: If (..) | ExprKind :: Loop (..) | ExprKind :: Match (..) | ExprKind :: Block (..) => true , ExprKind :: Index (..) | ExprKind :: Field (..) | ExprKind :: Unary (..) => false , ExprKind :: Struct (..) => true , ExprKind :: Array (..) => false , ExprKind :: Break (..) | ExprKind :: Continue (..) | ExprKind :: Ret (..) | ExprKind :: Become (..) => { false } ExprKind :: Assign (..) | ExprKind :: AssignOp (..) | ExprKind :: Yield (..) => false , ExprKind :: AddrOf (..) | ExprKind :: OffsetOf (..) | ExprKind :: InlineAsm (..) => false , ExprKind :: Cast (..) | ExprKind :: Closure (..) | ExprKind :: Tup (..) | ExprKind :: DropTemps (..) | ExprKind :: Let (..) => false , ExprKind :: UnsafeBinderCast (..) => false , ExprKind :: Type (..) | ExprKind :: Err (..) => false , } }
};
}
