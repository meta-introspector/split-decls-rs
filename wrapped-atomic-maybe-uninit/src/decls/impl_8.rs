macro_rules! deps {
    () => {
        AtomicMaybeUninit!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < T : Primitive > fmt :: Debug for AtomicMaybeUninit < T > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (core :: any :: type_name :: < Self > ()) } }
    };
}

impl_8!()