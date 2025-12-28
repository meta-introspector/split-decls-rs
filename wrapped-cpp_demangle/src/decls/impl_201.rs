macro_rules! deps {
    () => {
        LeafName!();
        ClassEnumType!();
        SubstitutionTable!();
        GetLeafName!();
    };
}

macro_rules! impl_201 {
    () => {
        deps!();
        impl < 'a > GetLeafName < 'a > for ClassEnumType { fn get_leaf_name (& 'a self , subs : & 'a SubstitutionTable) -> Option < LeafName < 'a > > { match * self { ClassEnumType :: Named (ref name) | ClassEnumType :: ElaboratedStruct (ref name) | ClassEnumType :: ElaboratedUnion (ref name) | ClassEnumType :: ElaboratedEnum (ref name) => name . get_leaf_name (subs) , } } }
    };
}

impl_201!();