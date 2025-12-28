macro_rules! deps {
    () => {
        Stmt!();
        StmtList!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < A > FromIterator < Stmt < A > > for StmtList < A > { fn from_iter < T > (iter : T) -> Self where T : IntoIterator < Item = Stmt < A > > , { Self { stmts : iter . into_iter () . collect () , } } }
    };
}

impl_29!();