macro_rules! deps {
    () => {
        GetLeafName!();
        LeafName!();
        UnscopedTemplateName!();
        SubstitutionTable!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < 'a > GetLeafName < 'a > for UnscopedTemplateName { fn get_leaf_name (& 'a self , subs : & 'a SubstitutionTable) -> Option < LeafName < 'a > > { self . 0 . get_leaf_name (subs) } }
    };
}

impl_93!();