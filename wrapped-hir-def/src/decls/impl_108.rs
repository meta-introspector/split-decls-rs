macro_rules! deps {
    () => {
        GenericDefId!();
        TypeParamId!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl TypeParamId { pub fn parent (& self) -> GenericDefId { self . 0 . parent } pub fn local_id (& self) -> LocalTypeOrConstParamId { self . 0 . local_id } }
    };
}

impl_108!()