macro_rules! BlockOrExpr {
    () => {
        # [doc = " The code snippets built up for derived code are sometimes used as blocks"] # [doc = " (e.g. in a function body) and sometimes used as expressions (e.g. in a match"] # [doc = " arm). This structure avoids committing to either form until necessary,"] # [doc = " avoiding the insertion of any unnecessary blocks."] # [doc = ""] # [doc = " The statements come before the expression."] pub (crate) struct BlockOrExpr (ThinVec < ast :: Stmt > , Option < Box < Expr > >) ;
    };
}

BlockOrExpr!()