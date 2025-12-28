macro_rules! closure_has_block_body {
    () => {
        fn closure_has_block_body (closure : & ast :: ClosureExpr) -> bool { matches ! (closure . body () , Some (ast :: Expr :: BlockExpr (_))) }
    };
}

closure_has_block_body!();