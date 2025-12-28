macro_rules! deps {
    () => {
        AnyValueId!();
    };
}

macro_rules! impl_609 {
    () => {
        deps!();
        impl std :: hash :: Hash for AnyValueId { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { self . type_id . hash (state) ; } }
    };
}

impl_609!();