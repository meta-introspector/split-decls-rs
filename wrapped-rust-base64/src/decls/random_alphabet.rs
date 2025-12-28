macro_rules! deps {
    () => {
        Alphabet!();
    };
}

macro_rules! random_alphabet {
    () => {
        deps!();
        pub fn random_alphabet < R : Rng > (rng : & mut R) -> & 'static alphabet :: Alphabet { ALPHABETS . choose (rng) . unwrap () }
    };
}

random_alphabet!()