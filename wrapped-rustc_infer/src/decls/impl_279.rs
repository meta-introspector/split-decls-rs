macro_rules! deps {
    () => {
        InferCtxt!();
    };
}

macro_rules! impl_279 {
    () => {
        deps!();
        impl < 'tcx > InferCtxt < 'tcx > { # [doc = " Given a [`hir::Block`], get the span of its last expression or"] # [doc = " statement, peeling off any inner blocks."] pub fn find_block_span (& self , block : & 'tcx hir :: Block < 'tcx >) -> Span { let block = block . innermost_block () ; if let Some (expr) = & block . expr { expr . span } else if let Some (stmt) = block . stmts . last () { stmt . span } else { block . span } } # [doc = " Given a [`hir::HirId`] for a block (or an expr of a block), get the span"] # [doc = " of its last expression or statement, peeling off any inner blocks."] pub fn find_block_span_from_hir_id (& self , hir_id : hir :: HirId) -> Span { match self . tcx . hir_node (hir_id) { hir :: Node :: Block (blk) | hir :: Node :: Expr (& hir :: Expr { kind : hir :: ExprKind :: Block (blk , _) , .. }) => { self . find_block_span (blk) } hir :: Node :: Expr (e) => e . span , _ => DUMMY_SP , } } }
    };
}

impl_279!();