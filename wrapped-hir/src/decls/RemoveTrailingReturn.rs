macro_rules! RemoveTrailingReturn {
    () => {
        # [derive (Debug)] pub struct RemoveTrailingReturn { pub return_expr : InFile < AstPtr < ast :: ReturnExpr > > , }
    };
}

RemoveTrailingReturn!();