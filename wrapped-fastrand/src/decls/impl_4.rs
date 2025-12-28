macro_rules! deps {
    () => {
        Rng!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl Clone for Rng { # [doc = " Clones the generator by creating a new generator with the same seed."] fn clone (& self) -> Rng { Rng :: with_seed (self . 0) } }
    };
}

impl_4!()