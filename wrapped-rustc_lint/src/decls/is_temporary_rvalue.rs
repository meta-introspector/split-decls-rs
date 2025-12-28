macro_rules! is_temporary_rvalue {
    () => {
        fn is_temporary_rvalue (expr : & Expr < '_ >) -> bool { match expr . kind { ExprKind :: ConstBlock (..) | ExprKind :: Repeat (..) | ExprKind :: Lit (..) => false , ExprKind :: Path (..) => false , ExprKind :: Call (..) | ExprKind :: MethodCall (..) | ExprKind :: Use (..) | ExprKind :: Binary (..) => true , ExprKind :: If (..) | ExprKind :: Loop (..) | ExprKind :: Match (..) | ExprKind :: Block (..) => true , ExprKind :: Index (..) | ExprKind :: Field (..) | ExprKind :: Unary (..) => false , ExprKind :: Struct (..) => true , ExprKind :: Array (..) => false , ExprKind :: Break (..) | ExprKind :: Continue (..) | ExprKind :: Ret (..) | ExprKind :: Become (..) => { false } ExprKind :: Assign (..) | ExprKind :: AssignOp (..) | ExprKind :: Yield (..) => false , ExprKind :: AddrOf (..) | ExprKind :: OffsetOf (..) | ExprKind :: InlineAsm (..) => false , ExprKind :: Cast (..) | ExprKind :: Closure (..) | ExprKind :: Tup (..) | ExprKind :: DropTemps (..) | ExprKind :: Let (..) => false , ExprKind :: UnsafeBinderCast (..) => false , ExprKind :: Type (..) | ExprKind :: Err (..) => false , } }
    };
}

is_temporary_rvalue!();