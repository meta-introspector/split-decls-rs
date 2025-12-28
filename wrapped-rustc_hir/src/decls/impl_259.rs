macro_rules! deps {
    () => {
        Term!();
        ConstArg!();
    };
}

macro_rules! impl_259 {
    () => {
        deps!();
        impl < 'hir > From < & 'hir ConstArg < 'hir > > for Term < 'hir > { fn from (c : & 'hir ConstArg < 'hir >) -> Self { Term :: Const (c) } }
    };
}

impl_259!();