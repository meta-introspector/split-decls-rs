macro_rules! deps {
    () => {
        BlockOrExpr!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl BlockOrExpr { pub (crate) fn new_stmts (stmts : ThinVec < ast :: Stmt >) -> BlockOrExpr { BlockOrExpr (stmts , None) } pub (crate) fn new_expr (expr : Box < Expr >) -> BlockOrExpr { BlockOrExpr (ThinVec :: new () , Some (expr)) } pub (crate) fn new_mixed (stmts : ThinVec < ast :: Stmt > , expr : Option < Box < Expr > >) -> BlockOrExpr { BlockOrExpr (stmts , expr) } fn into_block (mut self , cx : & ExtCtxt < '_ > , span : Span) -> Box < ast :: Block > { if let Some (expr) = self . 1 { self . 0 . push (cx . stmt_expr (expr)) ; } cx . block (span , self . 0) } fn into_expr (self , cx : & ExtCtxt < '_ > , span : Span) -> Box < Expr > { if self . 0 . is_empty () { match self . 1 { None => cx . expr_block (cx . block (span , ThinVec :: new ())) , Some (expr) => expr , } } else if let [stmt] = self . 0 . as_slice () && let ast :: StmtKind :: Expr (expr) = & stmt . kind && self . 1 . is_none () { expr . clone () } else { cx . expr_block (self . into_block (cx , span)) } } }
    };
}

impl_123!();