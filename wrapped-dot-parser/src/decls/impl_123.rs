macro_rules! deps {
    () => {
        StmtList!();
        Stmt!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl < 'a , A > IntoIterator for & 'a StmtList < A > { type Item = & 'a Stmt < A > ; type IntoIter = std :: slice :: Iter < 'a , Stmt < A > > ; fn into_iter (self) -> Self :: IntoIter { self . stmts . iter () } }
    };
}

impl_123!()