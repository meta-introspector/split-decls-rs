macro_rules! deps {
    () => {
        SubstitutionTable!();
        LeafName!();
        GetLeafName!();
        Name!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl < 'a > GetLeafName < 'a > for Name { fn get_leaf_name (& 'a self , subs : & 'a SubstitutionTable) -> Option < LeafName < 'a > > { match * self { Name :: UnscopedTemplate (ref templ , _) => templ . get_leaf_name (subs) , Name :: Nested (ref nested) => nested . get_leaf_name (subs) , Name :: Unscoped (ref unscoped) => unscoped . get_leaf_name (subs) , Name :: Local (ref local) => local . get_leaf_name (subs) , } } }
    };
}

impl_82!();