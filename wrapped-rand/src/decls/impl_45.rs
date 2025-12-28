macro_rules! deps {
    () => {
        StandardUniform!();
        Distribution!();
        Rng!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl Distribution < u64 > for StandardUniform { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> u64 { rng . next_u64 () } }
    };
}

impl_45!();