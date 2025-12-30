// Generated macro for impl_887 (impl)
macro_rules! Depcrate_tls_prfimpl_887 {
() => {
// Module: crate::tls_prf
// Provides: {"impl_887"}
// Dependencies: {}
impl Secret { # [doc = " Constructs a new `Secret` for use with the associated `P_hash` `Algorithm`."] # [doc = ""] # [doc = " # Errors"] # [doc = " * `Unspecified`: If `secret.is_empty() == true`."] pub fn new (algorithm : & 'static Algorithm , secret : & [u8]) -> Result < Self , Unspecified > { if secret . is_empty () { return Err (Unspecified) ; } let secret = Vec :: from (secret) . into_boxed_slice () ; Ok (Self { algorithm , secret }) } # [doc = " Calculates `len` bytes of TLS PRF using the configured [`Algorithm`], and returns [`Secret`] of length `len`."] # [doc = " See [RFC5246](https://datatracker.ietf.org/doc/html/rfc5246#section-5)"] # [doc = ""] # [doc = " # Errors"] # [doc = " * `Unspecified`: Returned if the PRF derivation fails."] pub fn derive (self , label : & [u8] , seed : & [u8] , output : usize) -> Result < Secret , Unspecified > { prf (self . algorithm , & self . secret , label , seed , None , output) } # [doc = " Calculates `len` bytes of TLS PRF using the configured [`Algorithm`], and returns [`Secret`] of length `len`."] # [doc = ""] # [doc = " In this method, `seed1` and `seed2` will be concatenated together: `seed1 || seed2`."] # [doc = ""] # [doc = " See [RFC5246](https://datatracker.ietf.org/doc/html/rfc5246#section-5)"] # [doc = ""] # [doc = " # Errors"] # [doc = " * `Unspecified`: Returned if the PRF derivation fails."] pub fn derive_with_seed_concatination (self , label : & [u8] , seed1 : & [u8] , seed2 : & [u8] , len : usize ,) -> Result < Secret , Unspecified > { prf (self . algorithm , & self . secret , label , seed1 , Some (seed2) , len) } }
};
}
