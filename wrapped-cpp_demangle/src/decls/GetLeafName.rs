macro_rules! deps {
    () => {
        LeafName!();
        SubstitutionTable!();
    };
}

macro_rules! GetLeafName {
    () => {
        deps!();
        # [doc = " Determine whether this AST node is some kind (potentially namespaced) name"] # [doc = " and if so get its leaf name."] pub (crate) trait GetLeafName < 'a > { fn get_leaf_name (& 'a self , subs : & 'a SubstitutionTable) -> Option < LeafName < 'a > > ; }
    };
}

GetLeafName!()