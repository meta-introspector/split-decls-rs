macro_rules! deps {
    () => {
        Stmt!();
        StmtList!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < 'a , A > IntoIterator for & 'a StmtList < A > { type Item = & 'a Stmt < A > ; type IntoIter = std :: slice :: Iter < 'a , Stmt < A > > ; fn into_iter (self) -> Self :: IntoIter { self . stmts . iter () } }
    };
}

impl_28!();