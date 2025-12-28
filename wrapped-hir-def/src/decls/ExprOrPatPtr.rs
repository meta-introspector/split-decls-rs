macro_rules! deps {
    () => {
        Pat!();
        Expr!();
    };
}

macro_rules! ExprOrPatPtr {
    () => {
        deps!();
        pub type ExprOrPatPtr = AstPtr < Either < ast :: Expr , ast :: Pat > > ;
    };
}

ExprOrPatPtr!()