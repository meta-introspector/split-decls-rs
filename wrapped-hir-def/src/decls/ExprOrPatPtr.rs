macro_rules! deps {
    () => {
        Expr!();
        Pat!();
    };
}

macro_rules! ExprOrPatPtr {
    () => {
        deps!();
        pub type ExprOrPatPtr = AstPtr < Either < ast :: Expr , ast :: Pat > > ;
    };
}

ExprOrPatPtr!();