macro_rules! deps {
    () => {
        Item!();
        OwnerNode!();
    };
}

macro_rules! impl_335 {
    () => {
        deps!();
        impl < 'hir > From < & 'hir Item < 'hir > > for OwnerNode < 'hir > { fn from (val : & 'hir Item < 'hir >) -> Self { OwnerNode :: Item (val) } }
    };
}

impl_335!()