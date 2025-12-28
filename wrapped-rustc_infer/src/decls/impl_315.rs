macro_rules! deps {
    () => {
        Obligation!();
    };
}

macro_rules! impl_315 {
    () => {
        deps!();
        impl < T : Hash > Hash for Obligation < '_ , T > { fn hash < H : Hasher > (& self , state : & mut H) -> () { self . param_env . hash (state) ; self . predicate . hash (state) ; } }
    };
}

impl_315!()