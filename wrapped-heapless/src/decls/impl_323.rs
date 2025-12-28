macro_rules! deps {
    () => {
        LenType!();
        VecInner!();
    };
}

macro_rules! impl_323 {
    () => {
        deps!();
        impl < T , LenT : LenType , S : VecStorage < T > + ? Sized > Eq for VecInner < T , LenT , S > where T : Eq { }
    };
}

impl_323!();