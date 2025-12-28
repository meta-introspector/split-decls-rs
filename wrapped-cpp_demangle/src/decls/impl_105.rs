macro_rules! deps {
    () => {
        Prefix!();
        SubstitutionTable!();
        TemplateParam!();
        LeafName!();
        Decltype!();
        GetLeafName!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl < 'a > GetLeafName < 'a > for Prefix { fn get_leaf_name (& 'a self , subs : & 'a SubstitutionTable) -> Option < LeafName < 'a > > { match * self { Prefix :: Nested (ref prefix , ref name) => name . get_leaf_name (subs) . or_else (| | prefix . get_leaf_name (subs)) , Prefix :: Unqualified (ref name) => name . get_leaf_name (subs) , Prefix :: Template (ref prefix , _) => prefix . get_leaf_name (subs) , Prefix :: DataMember (_ , ref name) => name . get_leaf_name (subs) , Prefix :: TemplateParam (_) | Prefix :: Decltype (_) => None , } } }
    };
}

impl_105!();