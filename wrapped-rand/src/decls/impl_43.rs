macro_rules! deps {
    () => {
        Distribution!();
        Rng!();
        StandardUniform!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl Distribution < u16 > for StandardUniform { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> u16 { rng . next_u32 () as u16 } }
    };
}

impl_43!();