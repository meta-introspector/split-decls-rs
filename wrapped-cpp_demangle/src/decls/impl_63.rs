macro_rules! deps {
    () => {
        GetLeafName!();
        LeafName!();
        SubstitutionTable!();
        NonSubstitution!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < 'a > GetLeafName < 'a > for NonSubstitution { fn get_leaf_name (& 'a self , subs : & 'a SubstitutionTable) -> Option < LeafName < 'a > > { subs . get_non_substitution (self . 0) . and_then (| ns | ns . get_leaf_name (subs)) } }
    };
}

impl_63!();