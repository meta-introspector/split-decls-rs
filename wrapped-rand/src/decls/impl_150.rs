macro_rules! deps {
    () => {
        Distribution!();
        Rng!();
        StandardUniform!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl Distribution < bool > for StandardUniform { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> bool { (rng . next_u32 () as i32) < 0 } }
    };
}

impl_150!();