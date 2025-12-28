macro_rules! deps {
    () => {
        GenericDefId!();
        ConstParamId!();
    };
}

macro_rules! impl_684 {
    () => {
        deps!();
        impl ConstParamId { pub fn parent (& self) -> GenericDefId { self . 0 . parent } pub fn local_id (& self) -> LocalTypeOrConstParamId { self . 0 . local_id } }
    };
}

impl_684!()