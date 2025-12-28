macro_rules! deps {
    () => {
        ImplItem!();
        OwnerNode!();
    };
}

macro_rules! impl_337 {
    () => {
        deps!();
        impl < 'hir > From < & 'hir ImplItem < 'hir > > for OwnerNode < 'hir > { fn from (val : & 'hir ImplItem < 'hir >) -> Self { OwnerNode :: ImplItem (val) } }
    };
}

impl_337!()