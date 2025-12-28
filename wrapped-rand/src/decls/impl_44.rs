macro_rules! deps {
    () => {
        Rng!();
        Distribution!();
        StandardUniform!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl Distribution < u32 > for StandardUniform { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> u32 { rng . next_u32 () } }
    };
}

impl_44!();