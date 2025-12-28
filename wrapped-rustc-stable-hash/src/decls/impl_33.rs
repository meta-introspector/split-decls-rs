macro_rules! deps {
    () => {
        ExtendedHasher!();
        StableHasher!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < H : ExtendedHasher + fmt :: Debug > fmt :: Debug for StableHasher < H > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{:?}" , self . state) } }
    };
}

impl_33!();