macro_rules! deps {
    () => {
        GeneralPurpose!();
    };
}

macro_rules! random_engine {
    () => {
        deps!();
        pub fn random_engine < R : Rng > (rng : & mut R) -> GeneralPurpose { let alphabet = random_alphabet (rng) ; let config = random_config (rng) ; GeneralPurpose :: new (alphabet , config) }
    };
}

random_engine!();