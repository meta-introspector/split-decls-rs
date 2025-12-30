// Generated macro for SecureRandom (trait)
macro_rules! Depcrate_randSecureRandom {
() => {
// Module: crate::rand
// Provides: {"SecureRandom"}
// Dependencies: {}
# [doc = " A secure random number generator."] pub trait SecureRandom : sealed :: SecureRandom { # [doc = " Fills `dest` with random bytes."] # [doc = ""] # [doc = " # Errors"] # [doc = " `error::Unspecified` if unable to fill `dest`."] fn fill (& self , dest : & mut [u8]) -> Result < () , Unspecified > ; }
};
}
