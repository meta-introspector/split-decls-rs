macro_rules! deps {
    () => {
        SubstitutionTable!();
        QualifiedBuiltin!();
        LeafName!();
        GetLeafName!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        impl < 'a > GetLeafName < 'a > for QualifiedBuiltin { fn get_leaf_name (& 'a self , _ : & 'a SubstitutionTable) -> Option < LeafName < 'a > > { None } }
    };
}

impl_181!();