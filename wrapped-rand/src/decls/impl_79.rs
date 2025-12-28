macro_rules! deps {
    () => {
        Distribution!();
        StandardUniform!();
        Rng!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl Distribution < bool > for StandardUniform { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> bool { (rng . next_u32 () as i32) < 0 } }
    };
}

impl_79!();