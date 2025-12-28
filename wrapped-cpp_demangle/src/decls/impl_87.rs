macro_rules! deps {
    () => {
        SubstitutionTable!();
        GetLeafName!();
        LeafName!();
        UnscopedName!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl < 'a > GetLeafName < 'a > for UnscopedName { fn get_leaf_name (& 'a self , subs : & 'a SubstitutionTable) -> Option < LeafName < 'a > > { match * self { UnscopedName :: Unqualified (ref name) | UnscopedName :: Std (ref name) => { name . get_leaf_name (subs) } } } }
    };
}

impl_87!();