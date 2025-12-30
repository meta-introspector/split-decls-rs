// Generated macro for digest (function)
macro_rules! Depcrate_high_level_hashdigest {
() => {
// Module: crate::high_level::hash
// Provides: {"digest"}
// Dependencies: {}
# [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Hashing using BLAKE2b-256."] pub fn digest (data : & [u8]) -> Result < Digest , UnknownCryptoError > { blake2b :: Hasher :: Blake2b256 . digest (data) }
};
}
