macro_rules! expr_requires_semi_to_be_stmt {
    () => {
        # [doc = " Does this expression require a semicolon to be treated"] # [doc = " as a statement? The negation of this: 'can this expression"] # [doc = " be used as a statement without a semicolon' -- is used"] # [doc = " as an early-bail-out in the parser so that, for instance,"] # [doc = "     if true {...} else {...}"] # [doc = "      |x| 5"] # [doc = " isn't parsed as (if true {...} else {...} | x) | 5"] fn expr_requires_semi_to_be_stmt (e : & hir :: Expr < '_ >) -> bool { ! matches ! (e . kind , hir :: ExprKind :: If (..) | hir :: ExprKind :: Match (..) | hir :: ExprKind :: Block (..) | hir :: ExprKind :: Loop (..)) }
    };
}

expr_requires_semi_to_be_stmt!();