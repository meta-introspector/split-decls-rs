macro_rules! deps {
    () => {
        Primitive!();
        AtomicMaybeUninit!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl < T : Primitive > fmt :: Debug for AtomicMaybeUninit < T > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (core :: any :: type_name :: < Self > ()) } }
    };
}

impl_95!()