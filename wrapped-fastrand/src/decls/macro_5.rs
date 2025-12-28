macro_rules! deps {
    () => {
        Rng!();
    };
}

macro_rules! macro_5 {
    () => {
        deps!();
        std :: thread_local ! { static RNG : Cell < Rng > = Cell :: new (Rng (random_seed () . unwrap_or (DEFAULT_RNG_SEED))) ; }
    };
}

macro_5!();