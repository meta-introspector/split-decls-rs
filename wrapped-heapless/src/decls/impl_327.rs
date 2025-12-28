macro_rules! deps {
    () => {
        LenType!();
        VecInner!();
    };
}

macro_rules! impl_327 {
    () => {
        deps!();
        impl < T , LenT : LenType , S : VecStorage < T > + ? Sized > ops :: DerefMut for VecInner < T , LenT , S > { fn deref_mut (& mut self) -> & mut Self :: Target { self . as_mut_slice () } }
    };
}

impl_327!();