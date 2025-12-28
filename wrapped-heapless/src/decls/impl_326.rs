macro_rules! deps {
    () => {
        LenType!();
        VecInner!();
    };
}

macro_rules! impl_326 {
    () => {
        deps!();
        impl < T , LenT : LenType , S : VecStorage < T > + ? Sized > ops :: Deref for VecInner < T , LenT , S > { type Target = [T] ; fn deref (& self) -> & Self :: Target { self . as_slice () } }
    };
}

impl_326!();