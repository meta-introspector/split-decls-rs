macro_rules! deps {
    () => {
        Expr!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl From < Box < Expr > > for Expr { fn from (value : Box < Expr >) -> Self { * value } }
    };
}

impl_87!()