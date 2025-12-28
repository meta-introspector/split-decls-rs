macro_rules! deps {
    () => {
        Rng!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl Default for Rng { # [doc = " Initialize the `Rng` from the system's random number generator."] # [doc = ""] # [doc = " This is equivalent to [`Rng::new()`]."] # [inline] fn default () -> Rng { Rng :: new () } }
    };
}

impl_3!();