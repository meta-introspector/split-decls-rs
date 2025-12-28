macro_rules! deps {
    () => {
        Expr!();
    };
}

macro_rules! ExprTopDownIterator {
    () => {
        deps!();
        # [doc = " The top down iterator for an expression."] pub struct ExprTopDownIterator { current : Option < Expr > , next : Option < Expr > , right_branches : Vec < Expr > , }
    };
}

ExprTopDownIterator!()