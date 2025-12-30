// Generated macro for random_alphabet (function)
macro_rules! Depcrate_testsrandom_alphabet {
() => {
// Module: crate::tests
// Provides: {"random_alphabet"}
// Dependencies: {}
pub fn random_alphabet < R : Rng > (rng : & mut R) -> & 'static alphabet :: Alphabet { ALPHABETS . choose (rng) . unwrap () }
};
}
