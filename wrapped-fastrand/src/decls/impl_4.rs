macro_rules! deps {
    () => {
        Rng!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl Rng { # [doc = " Creates a new random number generator."] # [inline] pub fn new () -> Rng { try_with_rng (Rng :: fork) . unwrap_or_else (| _ | Rng :: with_seed (0x4d595df4d0f33173)) } }
    };
}

impl_4!()