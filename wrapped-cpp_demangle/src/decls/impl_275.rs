macro_rules! deps {
    () => {
        LeafName!();
        LocalName!();
        SubstitutionTable!();
        GetLeafName!();
    };
}

macro_rules! impl_275 {
    () => {
        deps!();
        impl < 'a > GetLeafName < 'a > for LocalName { fn get_leaf_name (& 'a self , subs : & 'a SubstitutionTable) -> Option < LeafName < 'a > > { match * self { LocalName :: Relative (_ , None , _) => None , LocalName :: Relative (_ , Some (ref name) , _) | LocalName :: Default (_ , _ , ref name) => { name . get_leaf_name (subs) } } } }
    };
}

impl_275!()