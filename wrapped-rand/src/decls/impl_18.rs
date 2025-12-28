macro_rules! deps {
    () => {
        Rng!();
        Distribution!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < T , D : Distribution < T > + ? Sized > Distribution < T > for & D { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> T { (* self) . sample (rng) } }
    };
}

impl_18!()