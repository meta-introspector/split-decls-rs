// Generated macro for impl_167 (impl)
macro_rules! Depcrate_hazardous_hash_sha2_sha384impl_167 {
() => {
// Module: crate::hazardous::hash::sha2::sha384
// Provides: {"impl_167"}
// Dependencies: {}
impl Sha384 { # [doc = " Initialize a `Sha384` struct."] pub fn new () -> Self { Self { _state : State :: < WordU64 , V384 , SHA384_BLOCKSIZE , SHA384_OUTSIZE , N_CONSTS > :: _new () , } } # [doc = " Reset to `new()` state."] pub fn reset (& mut self) { self . _state . _reset () ; } # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Update state with `data`. This can be called multiple times."] pub fn update (& mut self , data : & [u8]) -> Result < () , UnknownCryptoError > { self . _state . _update (data) } # [doc = " Finalize the hash and put the final digest into `dest`."] pub (crate) fn _finalize_internal (& mut self , dest : & mut [u8]) -> Result < () , UnknownCryptoError > { self . _state . _finalize (dest) } # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Return a SHA384 digest."] pub fn finalize (& mut self) -> Result < Digest , UnknownCryptoError > { let mut digest = [0u8 ; SHA384_OUTSIZE] ; self . _finalize_internal (& mut digest) ? ; Ok (Digest :: from (digest)) } # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Calculate a SHA384 digest of some `data`."] pub fn digest (data : & [u8]) -> Result < Digest , UnknownCryptoError > { let mut ctx = Self :: new () ; ctx . update (data) ? ; ctx . finalize () } }
};
}
