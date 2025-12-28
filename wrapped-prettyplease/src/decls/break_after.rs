macro_rules! break_after {
    () => {
        pub fn break_after (expr : & Expr) -> bool { if let Expr :: Group (group) = expr { if let Expr :: Verbatim (verbatim) = group . expr . as_ref () { return ! verbatim . is_empty () ; } } true }
    };
}

break_after!()