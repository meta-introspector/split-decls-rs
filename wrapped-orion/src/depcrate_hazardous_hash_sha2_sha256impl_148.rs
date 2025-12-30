// Generated macro for impl_148 (impl)
macro_rules! Depcrate_hazardous_hash_sha2_sha256impl_148 {
() => {
// Module: crate::hazardous::hash::sha2::sha256
// Provides: {"impl_148"}
// Dependencies: {}
impl Sha256 { # [doc = " Initialize a `Sha256` struct."] pub fn new () -> Self { Self { _state : State :: < WordU32 , V256 , SHA256_BLOCKSIZE , SHA256_OUTSIZE , N_CONSTS > :: _new () , } } # [doc = " Reset to `new()` state."] pub fn reset (& mut self) { self . _state . _reset () ; } # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Update state with `data`. This can be called multiple times."] pub fn update (& mut self , data : & [u8]) -> Result < () , UnknownCryptoError > { self . _state . _update (data) } # [doc = " Finalize the hash and put the final digest into `dest`."] pub (crate) fn _finalize_internal (& mut self , dest : & mut [u8]) -> Result < () , UnknownCryptoError > { self . _state . _finalize (dest) } # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Return a SHA256 digest."] pub fn finalize (& mut self) -> Result < Digest , UnknownCryptoError > { let mut digest = [0u8 ; SHA256_OUTSIZE] ; self . _finalize_internal (& mut digest) ? ; Ok (Digest :: from (digest)) } # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Calculate a SHA256 digest of some `data`."] pub fn digest (data : & [u8]) -> Result < Digest , UnknownCryptoError > { let mut ctx = Self :: new () ; ctx . update (data) ? ; ctx . finalize () } }
};
}
