macro_rules! deps {
    () => {
        Distribution!();
        Rng!();
        Bernoulli!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl Distribution < bool > for Bernoulli { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> bool { if self . p_int == ALWAYS_TRUE { return true ; } let v : u64 = rng . random () ; v < self . p_int } }
    };
}

impl_14!()