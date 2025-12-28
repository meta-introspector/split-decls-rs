macro_rules! requires_semi_to_be_stmt {
    () => {
        pub (crate) fn requires_semi_to_be_stmt (expr : & Expr) -> bool { match expr { Expr :: Macro (expr) => ! matches ! (expr . mac . delimiter , MacroDelimiter :: Brace (_)) , _ => requires_comma_to_be_match_arm (expr) , } }
    };
}

requires_semi_to_be_stmt!()