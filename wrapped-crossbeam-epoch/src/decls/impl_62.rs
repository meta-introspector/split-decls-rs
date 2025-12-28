macro_rules! deps {
    () => {
        Shared!();
        Pointer!();
        Pointable!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < T : ? Sized + Pointable > fmt :: Pointer for Shared < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Pointer :: fmt (& (unsafe { self . deref () as * const _ }) , f) } }
    };
}

impl_62!()