macro_rules! deps {
    () => {
        TypeParamId!();
        GenericDefId!();
    };
}

macro_rules! impl_680 {
    () => {
        deps!();
        impl TypeParamId { pub fn parent (& self) -> GenericDefId { self . 0 . parent } pub fn local_id (& self) -> LocalTypeOrConstParamId { self . 0 . local_id } }
    };
}

impl_680!();