macro_rules! deps {
    () => {
        AssignOpKind!();
    };
}

macro_rules! AssignOp {
    () => {
        deps!();
        pub type AssignOp = Spanned < AssignOpKind > ;
    };
}

AssignOp!()