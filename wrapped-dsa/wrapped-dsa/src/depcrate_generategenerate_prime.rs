// Generated macro for generate_prime (function)
macro_rules! Depcrate_generategenerate_prime {
() => {
// Module: crate::generate
// Provides: {"generate_prime"}
// Dependencies: {}
# [doc = " Generate a prime number using a cryptographically secure pseudo-random number generator"] # [doc = ""] # [doc = " This wrapper function mainly exists to enforce the [`CryptoRng`](rand::CryptoRng) requirement (I might otherwise forget it)"] # [inline] fn generate_prime < R : CryptoRng + ? Sized > (bit_length : u32 , rng : & mut R) -> BoxedUint { random_prime (rng , Flavor :: Any , bit_length) }
};
}
