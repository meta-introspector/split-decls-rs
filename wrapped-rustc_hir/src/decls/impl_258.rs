macro_rules! deps {
    () => {
        Ty!();
        Term!();
    };
}

macro_rules! impl_258 {
    () => {
        deps!();
        impl < 'hir > From < & 'hir Ty < 'hir > > for Term < 'hir > { fn from (ty : & 'hir Ty < 'hir >) -> Self { Term :: Ty (ty) } }
    };
}

impl_258!();