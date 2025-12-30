// Generated macro for impl_168 (impl)
macro_rules! Depcrate_hazardous_hash_sha2_sha384impl_168 {
() => {
// Module: crate::hazardous::hash::sha2::sha384
// Provides: {"impl_168"}
// Dependencies: {}
impl crate :: hazardous :: mac :: hmac :: HmacHashFunction for Sha384 { # [doc = " The blocksize of the hash function."] const _BLOCKSIZE : usize = SHA384_BLOCKSIZE ; # [doc = " The output size of the hash function."] const _OUTSIZE : usize = SHA384_OUTSIZE ; # [doc = " Create a new instance of the hash function."] fn _new () -> Self { Self :: new () } # [doc = " Update the internal state with `data`."] fn _update (& mut self , data : & [u8]) -> Result < () , UnknownCryptoError > { self . update (data) } # [doc = " Finalize the hash and put the final digest into `dest`."] fn _finalize (& mut self , dest : & mut [u8]) -> Result < () , UnknownCryptoError > { self . _finalize_internal (dest) } # [doc = " Compute a digest of `data` and copy it into `dest`."] fn _digest (data : & [u8] , dest : & mut [u8]) -> Result < () , UnknownCryptoError > { let mut ctx = Self :: new () ; ctx . update (data) ? ; ctx . _finalize_internal (dest) } # [cfg (test)] fn compare_state_to_other (& self , other : & Self) { self . _state . compare_state_to_other (& other . _state) ; } }
};
}
