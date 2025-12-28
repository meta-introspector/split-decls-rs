macro_rules! deps {
    () => {
        Pointable!();
        Atomic!();
        Pointer!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < T : ? Sized + Pointable > fmt :: Pointer for Atomic < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let data = self . data . load (Ordering :: SeqCst) ; let (raw , _) = decompose_tag :: < T > (data) ; fmt :: Pointer :: fmt (& (unsafe { T :: deref (raw) as * const _ }) , f) } }
    };
}

impl_24!()