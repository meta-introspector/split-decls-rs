// Generated macro for SecureRandom (trait)
macro_rules! Depcrate_randSecureRandom {
() => {
// Module: crate::rand
// Provides: {"SecureRandom"}
// Dependencies: {}
# [doc = " A secure random number generator."] pub trait SecureRandom : sealed :: SecureRandom { # [doc = " Fills `dest` with random bytes."] fn fill (& self , dest : & mut [u8]) -> Result < () , error :: Unspecified > ; }
};
}
