macro_rules! deps {
    () => {
        Term!();
        Ty!();
    };
}

macro_rules! impl_258 {
    () => {
        deps!();
        impl < 'hir > From < & 'hir Ty < 'hir > > for Term < 'hir > { fn from (ty : & 'hir Ty < 'hir >) -> Self { Term :: Ty (ty) } }
    };
}

impl_258!()