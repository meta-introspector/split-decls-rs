macro_rules! deps {
    () => {
        StmtList!();
        Stmt!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < A > IntoIterator for StmtList < A > { type Item = Stmt < A > ; type IntoIter = std :: vec :: IntoIter < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { self . stmts . into_iter () } }
    };
}

impl_27!();