macro_rules! deps {
    () => {
        LeafName!();
        GetLeafName!();
        BuiltinType!();
        SubstitutionTable!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        impl < 'a > GetLeafName < 'a > for BuiltinType { fn get_leaf_name (& 'a self , _ : & 'a SubstitutionTable) -> Option < LeafName < 'a > > { None } }
    };
}

impl_178!();