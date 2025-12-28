macro_rules! deps {
    () => {
        Substitutable!();
        LeafName!();
        SubstitutionTable!();
        UnscopedTemplateName!();
        GetLeafName!();
        Type!();
        Prefix!();
    };
}

macro_rules! impl_333 {
    () => {
        deps!();
        impl < 'a > ast :: GetLeafName < 'a > for Substitutable { fn get_leaf_name (& 'a self , subs : & 'a SubstitutionTable) -> Option < ast :: LeafName < 'a > > { match * self { Substitutable :: UnscopedTemplateName (ref name) => name . get_leaf_name (subs) , Substitutable :: Prefix (ref prefix) => prefix . get_leaf_name (subs) , Substitutable :: Type (ref ty) => ty . get_leaf_name (subs) , _ => None , } } }
    };
}

impl_333!();