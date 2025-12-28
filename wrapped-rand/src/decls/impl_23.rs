macro_rules! deps {
    () => {
        Distribution!();
        Rng!();
        Map!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < D , F , T , S > Distribution < S > for Map < D , F , T , S > where D : Distribution < T > , F : Fn (T) -> S , { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> S { (self . func) (self . distr . sample (rng)) } }
    };
}

impl_23!();