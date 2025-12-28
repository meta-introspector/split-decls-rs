macro_rules! deps {
    () => {
        TypeName!();
    };
}

macro_rules! impl_428 {
    () => {
        deps!();
        impl MemberRefParent { pub fn type_name (& self) -> TypeName { match self { Self :: TypeDef (row) => row . type_name () , Self :: TypeRef (row) => row . type_name () , } } pub fn name (& self) -> & 'static str { match self { Self :: TypeDef (row) => row . name () , Self :: TypeRef (row) => row . name () , } } }
    };
}

impl_428!();