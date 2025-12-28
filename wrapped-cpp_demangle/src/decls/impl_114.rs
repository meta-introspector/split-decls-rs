macro_rules! deps {
    () => {
        LeafName!();
        SourceName!();
        GetLeafName!();
        UnqualifiedName!();
        SubstitutionTable!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl < 'a > GetLeafName < 'a > for UnqualifiedName { fn get_leaf_name (& 'a self , subs : & 'a SubstitutionTable) -> Option < LeafName < 'a > > { match * self { UnqualifiedName :: Operator (..) | UnqualifiedName :: CtorDtor (..) => None , UnqualifiedName :: UnnamedType (ref name , _) => Some (LeafName :: UnnamedType (name)) , UnqualifiedName :: ClosureType (ref closure , _) => closure . get_leaf_name (subs) , UnqualifiedName :: Source (ref name , _) | UnqualifiedName :: LocalSourceName (ref name , ..) => Some (LeafName :: SourceName (name)) , } } }
    };
}

impl_114!();