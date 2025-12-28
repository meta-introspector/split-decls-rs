macro_rules! deps {
    () => {
        ComObjectInner!();
        ComObject!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl < T : ComObjectInner + core :: hash :: Hash > core :: hash :: Hash for ComObject < T > { fn hash < H : core :: hash :: Hasher > (& self , state : & mut H) { self . get () . hash (state) ; } }
    };
}

impl_116!()