macro_rules! deps {
    () => {
        Expr!();
    };
}

macro_rules! ExprPtr {
    () => {
        deps!();
        pub type ExprPtr = AstPtr < ast :: Expr > ;
    };
}

ExprPtr!()