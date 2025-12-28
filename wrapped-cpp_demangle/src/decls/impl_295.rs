macro_rules! deps {
    () => {
        LeafName!();
        GetLeafName!();
        SubstitutionTable!();
    };
}

macro_rules! impl_295 {
    () => {
        deps!();
        impl < 'a > GetLeafName < 'a > for WellKnownComponent { fn get_leaf_name (& 'a self , _ : & 'a SubstitutionTable) -> Option < LeafName < 'a > > { match * self { WellKnownComponent :: Std => None , _ => Some (LeafName :: WellKnownComponent (self)) , } } }
    };
}

impl_295!();