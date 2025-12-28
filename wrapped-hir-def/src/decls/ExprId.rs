macro_rules! deps {
    () => {
        Expr!();
    };
}

macro_rules! ExprId {
    () => {
        deps!();
        pub type ExprId = Idx < Expr > ;
    };
}

ExprId!();