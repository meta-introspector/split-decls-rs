macro_rules! deps {
    () => {
        Type!();
        SubstitutionTable!();
        LeafName!();
        GetLeafName!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl < 'a > GetLeafName < 'a > for Type { fn get_leaf_name (& 'a self , subs : & 'a SubstitutionTable) -> Option < LeafName < 'a > > { match * self { Type :: ClassEnum (ref cls_enum_ty) => cls_enum_ty . get_leaf_name (subs) , _ => None , } } }
    };
}

impl_163!()