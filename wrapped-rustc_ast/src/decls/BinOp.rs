macro_rules! deps {
    () => {
        BinOpKind!();
    };
}

macro_rules! BinOp {
    () => {
        deps!();
        pub type BinOp = Spanned < BinOpKind > ;
    };
}

BinOp!();