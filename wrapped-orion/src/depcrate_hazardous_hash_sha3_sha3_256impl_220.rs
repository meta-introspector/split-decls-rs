// Generated macro for impl_220 (impl)
macro_rules! Depcrate_hazardous_hash_sha3_sha3_256impl_220 {
() => {
// Module: crate::hazardous::hash::sha3::sha3_256
// Provides: {"impl_220"}
// Dependencies: {}
impl Sha3_256 { # [doc = " Initialize a `Sha3_256` struct."] pub fn new () -> Self { Self { _state : Sha3 :: < { SHA3_256_RATE } > :: _new (64) , } } # [doc = " Reset to `new()` state."] pub fn reset (& mut self) { self . _state . _reset () ; } # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Update state with `data`. This can be called multiple times."] pub fn update (& mut self , data : & [u8]) -> Result < () , UnknownCryptoError > { self . _state . _update (data) } # [doc = " Finalize the hash and put the final digest into `dest`."] pub (crate) fn _finalize_internal (& mut self , dest : & mut [u8]) -> Result < () , UnknownCryptoError > { self . _state . _finalize (dest) } # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Return a SHA3-256 digest."] pub fn finalize (& mut self) -> Result < Digest , UnknownCryptoError > { let mut digest = [0u8 ; SHA3_256_OUTSIZE] ; self . _finalize_internal (& mut digest) ? ; Ok (Digest :: from (digest)) } # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Calculate a SHA3-256 digest of some `data`."] pub fn digest (data : & [u8]) -> Result < Digest , UnknownCryptoError > { let mut ctx = Self :: new () ; ctx . update (data) ? ; ctx . finalize () } }
};
}
