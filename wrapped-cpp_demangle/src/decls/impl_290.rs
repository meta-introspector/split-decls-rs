macro_rules! deps {
    () => {
        SubstitutionTable!();
        LeafName!();
        SourceName!();
        DataMemberPrefix!();
        GetLeafName!();
    };
}

macro_rules! impl_290 {
    () => {
        deps!();
        impl < 'a > GetLeafName < 'a > for DataMemberPrefix { # [inline] fn get_leaf_name (& 'a self , _ : & 'a SubstitutionTable) -> Option < LeafName < 'a > > { Some (LeafName :: SourceName (& self . 0)) } }
    };
}

impl_290!()