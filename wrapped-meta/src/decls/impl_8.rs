macro_rules! deps {
    () => {
        ExprTopDownIterator!();
        Expr!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl Iterator for ExprTopDownIterator { type Item = Expr ; fn next (& mut self) -> Option < Self :: Item > { let result = self . current . take () ; if let Some (expr) = self . next . take () { self . iterate_expr (expr) ; } else if let Some (expr) = self . right_branches . pop () { self . iterate_expr (expr) ; } result } }
    };
}

impl_8!();