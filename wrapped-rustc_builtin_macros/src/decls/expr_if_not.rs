macro_rules! expr_if_not {
    () => {
        fn expr_if_not (cx : & ExtCtxt < '_ > , span : Span , cond : Box < Expr > , then : Box < Expr > , els : Option < Box < Expr > > ,) -> Box < Expr > { cx . expr_if (span , cx . expr (span , ExprKind :: Unary (UnOp :: Not , cond)) , then , els) }
    };
}

expr_if_not!()