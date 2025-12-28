macro_rules! deps {
    () => {
        WalkExpandedExprCtx!();
    };
}

macro_rules! impl_201 {
    () => {
        deps!();
        impl < 'a > WalkExpandedExprCtx < 'a > { fn new (sema : & 'a Semantics < 'a , RootDatabase >) -> Self { Self { sema , depth : 0 , check_ctx : & is_closure_or_blk_with_modif } } fn with_check_ctx (& self , check_ctx : & 'static dyn Fn (& ast :: Expr) -> bool) -> Self { Self { check_ctx , .. * self } } fn walk (& mut self , expr : & ast :: Expr , cb : & mut dyn FnMut (usize , ast :: Expr)) { preorder_expr_with_ctx_checker (expr , self . check_ctx , & mut | ev : WalkEvent < ast :: Expr > | { match ev { syntax :: WalkEvent :: Enter (expr) => { cb (self . depth , expr . clone ()) ; if Self :: should_change_depth (& expr) { self . depth += 1 ; } if let ast :: Expr :: MacroExpr (expr) = expr && let Some (expanded) = expr . macro_call () . and_then (| call | self . sema . expand_macro_call (& call)) { match_ast ! { match (expanded . value) { ast :: MacroStmts (it) => { self . handle_expanded (it , cb) ; } , ast :: Expr (it) => { self . walk (& it , cb) ; } , _ => { } } } } } syntax :: WalkEvent :: Leave (expr) if Self :: should_change_depth (& expr) => { self . depth -= 1 ; } _ => { } } false }) } fn handle_expanded (& mut self , expanded : ast :: MacroStmts , cb : & mut dyn FnMut (usize , ast :: Expr)) { if let Some (expr) = expanded . expr () { self . walk (& expr , cb) ; } for stmt in expanded . statements () { if let ast :: Stmt :: ExprStmt (stmt) = stmt && let Some (expr) = stmt . expr () { self . walk (& expr , cb) ; } } } fn should_change_depth (expr : & ast :: Expr) -> bool { match expr { ast :: Expr :: LoopExpr (_) | ast :: Expr :: WhileExpr (_) | ast :: Expr :: ForExpr (_) => true , ast :: Expr :: BlockExpr (blk) if blk . label () . is_some () => true , _ => false , } } fn is_async_const_block_or_closure (expr : & ast :: Expr) -> bool { match expr { ast :: Expr :: BlockExpr (b) => matches ! (b . modifier () , Some (ast :: BlockModifier :: Async (_) | ast :: BlockModifier :: Const (_))) , ast :: Expr :: ClosureExpr (_) => true , _ => false , } } }
    };
}

impl_201!()