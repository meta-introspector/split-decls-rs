macro_rules! deps {
    () => {
        Crate!();
        OwnerNode!();
        ImplItem!();
        Item!();
        TraitItem!();
        Node!();
        ForeignItem!();
    };
}

macro_rules! impl_339 {
    () => {
        deps!();
        impl < 'hir > From < OwnerNode < 'hir > > for Node < 'hir > { fn from (val : OwnerNode < 'hir >) -> Self { match val { OwnerNode :: Item (n) => Node :: Item (n) , OwnerNode :: ForeignItem (n) => Node :: ForeignItem (n) , OwnerNode :: ImplItem (n) => Node :: ImplItem (n) , OwnerNode :: TraitItem (n) => Node :: TraitItem (n) , OwnerNode :: Crate (n) => Node :: Crate (n) , OwnerNode :: Synthetic => Node :: Synthetic , } } }
    };
}

impl_339!();