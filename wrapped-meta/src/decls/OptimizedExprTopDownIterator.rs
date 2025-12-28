macro_rules! deps {
    () => {
        OptimizedExpr!();
    };
}

macro_rules! OptimizedExprTopDownIterator {
    () => {
        deps!();
        # [doc = " A top-down iterator over an `OptimizedExpr`."] pub struct OptimizedExprTopDownIterator { current : Option < OptimizedExpr > , next : Option < OptimizedExpr > , right_branches : Vec < OptimizedExpr > , }
    };
}

OptimizedExprTopDownIterator!()