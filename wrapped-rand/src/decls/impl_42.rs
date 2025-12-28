macro_rules! deps {
    () => {
        Rng!();
        StandardUniform!();
        Distribution!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl Distribution < u8 > for StandardUniform { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> u8 { rng . next_u32 () as u8 } }
    };
}

impl_42!();