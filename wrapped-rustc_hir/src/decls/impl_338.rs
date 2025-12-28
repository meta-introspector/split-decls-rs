macro_rules! deps {
    () => {
        TraitItem!();
        OwnerNode!();
    };
}

macro_rules! impl_338 {
    () => {
        deps!();
        impl < 'hir > From < & 'hir TraitItem < 'hir > > for OwnerNode < 'hir > { fn from (val : & 'hir TraitItem < 'hir >) -> Self { OwnerNode :: TraitItem (val) } }
    };
}

impl_338!();