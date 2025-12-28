macro_rules! deps {
    () => {
        GetLeafName!();
        Type!();
        SubstitutionTable!();
        LeafName!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl < 'a > GetLeafName < 'a > for Type { fn get_leaf_name (& 'a self , subs : & 'a SubstitutionTable) -> Option < LeafName < 'a > > { match * self { Type :: ClassEnum (ref cls_enum_ty) => cls_enum_ty . get_leaf_name (subs) , _ => None , } } }
    };
}

impl_163!();