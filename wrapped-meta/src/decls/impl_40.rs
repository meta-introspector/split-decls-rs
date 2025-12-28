macro_rules! deps {
    () => {
        OptimizedExprTopDownIterator!();
        OptimizedExpr!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl Iterator for OptimizedExprTopDownIterator { type Item = OptimizedExpr ; fn next (& mut self) -> Option < Self :: Item > { let result = self . current . take () ; if let Some (expr) = self . next . take () { self . iterate_expr (expr) ; } else if let Some (expr) = self . right_branches . pop () { self . iterate_expr (expr) ; } result } }
    };
}

impl_40!()