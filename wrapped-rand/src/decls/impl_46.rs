macro_rules! deps {
    () => {
        StandardUniform!();
        Rng!();
        Distribution!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl Distribution < u128 > for StandardUniform { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> u128 { let x = u128 :: from (rng . next_u64 ()) ; let y = u128 :: from (rng . next_u64 ()) ; (y << 64) | x } }
    };
}

impl_46!()