macro_rules! seeded_rng {
    () => {
        fn seeded_rng () -> impl rand :: Rng { rngs :: SmallRng :: from_entropy () }
    };
}

seeded_rng!();