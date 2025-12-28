macro_rules! deps {
    () => {
        MakeMaybeUninit!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl < T , const N : usize > MakeMaybeUninit < T , N > { pub (crate) const VALUE : MaybeUninit < T > = MaybeUninit :: uninit () ; pub (crate) const ARRAY : [MaybeUninit < T > ; N] = [Self :: VALUE ; N] ; }
    };
}

impl_113!()