macro_rules! deps {
    () => {
        CheckInlineAssembly!();
        ItemKind!();
    };
}

macro_rules! impl_294 {
    () => {
        deps!();
        impl CheckInlineAssembly { fn check_expr < 'tcx > (& mut self , expr : & 'tcx hir :: Expr < 'tcx > , span : Span) { match expr . kind { ExprKind :: ConstBlock (..) | ExprKind :: Array (..) | ExprKind :: Call (..) | ExprKind :: MethodCall (..) | ExprKind :: Use (..) | ExprKind :: Tup (..) | ExprKind :: Binary (..) | ExprKind :: Unary (..) | ExprKind :: Lit (..) | ExprKind :: Cast (..) | ExprKind :: Type (..) | ExprKind :: UnsafeBinderCast (..) | ExprKind :: Loop (..) | ExprKind :: Match (..) | ExprKind :: If (..) | ExprKind :: Closure { .. } | ExprKind :: Assign (..) | ExprKind :: AssignOp (..) | ExprKind :: Field (..) | ExprKind :: Index (..) | ExprKind :: Path (..) | ExprKind :: AddrOf (..) | ExprKind :: Let (..) | ExprKind :: Break (..) | ExprKind :: Continue (..) | ExprKind :: Ret (..) | ExprKind :: OffsetOf (..) | ExprKind :: Become (..) | ExprKind :: Struct (..) | ExprKind :: Repeat (..) | ExprKind :: Yield (..) => { self . items . push ((ItemKind :: NonAsm , span)) ; } ExprKind :: InlineAsm (asm) => match asm . asm_macro { rustc_ast :: AsmMacro :: Asm => { self . items . push ((ItemKind :: InlineAsm , span)) ; } rustc_ast :: AsmMacro :: NakedAsm => { self . items . push ((ItemKind :: NakedAsm , span)) ; } rustc_ast :: AsmMacro :: GlobalAsm => { span_bug ! (span , "`global_asm!` is not allowed in this position") } } , ExprKind :: DropTemps (..) | ExprKind :: Block (..) => { hir :: intravisit :: walk_expr (self , expr) ; } ExprKind :: Err (_) => { self . items . push ((ItemKind :: Err , span)) ; } } } }
    };
}

impl_294!();