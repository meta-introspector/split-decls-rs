macro_rules! deps {
    () => {
        LeafName!();
        ClosureTypeName!();
        SubstitutionTable!();
        GetLeafName!();
    };
}

macro_rules! impl_282 {
    () => {
        deps!();
        impl < 'a > GetLeafName < 'a > for ClosureTypeName { # [inline] fn get_leaf_name (& 'a self , _ : & 'a SubstitutionTable) -> Option < LeafName < 'a > > { Some (LeafName :: Closure (self)) } }
    };
}

impl_282!();