// Generated macro for HmacHashFunction (trait)
macro_rules! Depcrate_hazardous_mac_hmacHmacHashFunction {
() => {
// Module: crate::hazardous::mac::hmac
// Provides: {"HmacHashFunction"}
// Dependencies: {}
# [doc = " A trait used to define a cryptographic hash function used by HMAC."] pub (crate) trait HmacHashFunction : Clone { # [doc = " The blocksize of the hash function."] const _BLOCKSIZE : usize ; # [doc = " The output size of the hash function."] const _OUTSIZE : usize ; # [doc = " Create a new instance of the hash function."] fn _new () -> Self ; # [doc = " Update the internal state with `data`."] fn _update (& mut self , data : & [u8]) -> Result < () , UnknownCryptoError > ; # [doc = " Finalize the hash and put the final digest into `dest`."] fn _finalize (& mut self , dest : & mut [u8]) -> Result < () , UnknownCryptoError > ; # [doc = " Compute a digest of `data` and copy it into `dest`."] fn _digest (data : & [u8] , dest : & mut [u8]) -> Result < () , UnknownCryptoError > ; # [cfg (test)] # [doc = " Compare two Sha2 state objects to check if their fields"] # [doc = " are the same."] fn compare_state_to_other (& self , other : & Self) ; }
};
}
