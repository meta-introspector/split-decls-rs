macro_rules! simple_block {
    () => {
        pub fn simple_block (expr : & Expr) -> Option < & ExprBlock > { if let Expr :: Block (expr) = expr { if expr . attrs . is_empty () && expr . label . is_none () { return Some (expr) ; } } None }
    };
}

simple_block!();