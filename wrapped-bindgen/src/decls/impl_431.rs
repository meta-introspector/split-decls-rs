macro_rules! deps {
    () => {
        Reader!();
        TypeName!();
    };
}

macro_rules! impl_431 {
    () => {
        deps!();
        impl TypeDefOrRef { pub fn type_name (& self) -> TypeName { match self { Self :: TypeDef (row) => row . type_name () , Self :: TypeRef (row) => row . type_name () , rest => panic ! ("{rest:?}") , } } pub fn reader (& self) -> & 'static Reader { match self { Self :: TypeDef (row) => row . reader () , Self :: TypeRef (row) => row . reader () , Self :: TypeSpec (row) => row . reader () , } } }
    };
}

impl_431!()