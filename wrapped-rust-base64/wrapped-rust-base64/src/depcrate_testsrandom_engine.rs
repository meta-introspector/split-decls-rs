// Generated macro for random_engine (function)
macro_rules! Depcrate_testsrandom_engine {
() => {
// Module: crate::tests
// Provides: {"random_engine"}
// Dependencies: {}
pub fn random_engine < R : Rng > (rng : & mut R) -> GeneralPurpose { let alphabet = random_alphabet (rng) ; let config = random_config (rng) ; GeneralPurpose :: new (alphabet , config) }
};
}
