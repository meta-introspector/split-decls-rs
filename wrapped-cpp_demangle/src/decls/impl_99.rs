macro_rules! deps {
    () => {
        GetLeafName!();
        NestedName!();
        LeafName!();
        SubstitutionTable!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl < 'a > GetLeafName < 'a > for NestedName { fn get_leaf_name (& 'a self , subs : & 'a SubstitutionTable) -> Option < LeafName < 'a > > { match * self { NestedName :: Unqualified (_ , _ , ref prefix , ref name) | NestedName :: UnqualifiedExplicitObject (ref prefix , ref name , _) => name . get_leaf_name (subs) . or_else (| | prefix . as_ref () . and_then (| p | p . get_leaf_name (subs))) , NestedName :: Template (_ , _ , ref prefix) | NestedName :: TemplateExplicitObject (ref prefix , _) => prefix . get_leaf_name (subs) , } } }
    };
}

impl_99!();