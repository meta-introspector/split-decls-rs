macro_rules! deps {
    () => {
        Rng!();
        Iter!();
        Distribution!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < D , R , T > iter :: FusedIterator for Iter < D , R , T > where D : Distribution < T > , R : Rng , { }
    };
}

impl_21!()