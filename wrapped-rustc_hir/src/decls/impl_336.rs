macro_rules! deps {
    () => {
        OwnerNode!();
        ForeignItem!();
    };
}

macro_rules! impl_336 {
    () => {
        deps!();
        impl < 'hir > From < & 'hir ForeignItem < 'hir > > for OwnerNode < 'hir > { fn from (val : & 'hir ForeignItem < 'hir >) -> Self { OwnerNode :: ForeignItem (val) } }
    };
}

impl_336!()