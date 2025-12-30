// Generated macro for impl_158 (impl)
macro_rules! Depcrate_cmacimpl_158 {
() => {
// Module: crate::cmac
// Provides: {"impl_158"}
// Dependencies: {}
impl Key { # [doc = " Generate a CMAC signing key using the given algorithm with a"] # [doc = " random value."] # [doc = ""] # [doc = " # Errors"] # [doc = " `error::Unspecified` if random generation or key construction fails."] pub fn generate (algorithm : Algorithm) -> Result < Self , Unspecified > { let mut key_bytes = vec ! [0u8 ; algorithm . key_len ()] ; rand :: fill (& mut key_bytes) ? ; Self :: new (algorithm , & key_bytes) } # [doc = " Construct a CMAC signing key using the given algorithm and key value."] # [doc = ""] # [doc = " `key_value` should be a value generated using a secure random number"] # [doc = " generator or derived from a random key by a key derivation function."] # [doc = ""] # [doc = " # Errors"] # [doc = " `error::Unspecified` if the key length doesn't match the algorithm or if CMAC context"] # [doc = " initialization fails."] pub fn new (algorithm : Algorithm , key_value : & [u8]) -> Result < Self , Unspecified > { if key_value . len () != algorithm . key_len () { return Err (Unspecified) ; } let mut ctx = LcPtr :: new (unsafe { CMAC_CTX_new () }) ? ; unsafe { let cipher = algorithm . id . evp_cipher () ; if 1 != CMAC_Init (* ctx . as_mut () , key_value . as_ptr () . cast () , key_value . len () , * cipher , null_mut () ,) { return Err (Unspecified) ; } } Ok (Self { algorithm , ctx }) } # [doc = " The algorithm for the key."] # [inline] # [must_use] pub fn algorithm (& self) -> Algorithm { self . algorithm } }
};
}
