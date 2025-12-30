// Generated macro for impl_327 (impl)
macro_rules! Depcrate_hkdfimpl_327 {
() => {
// Module: crate::hkdf
// Provides: {"impl_327"}
// Dependencies: {}
impl Prk { # [doc = " Construct a new `Prk` directly with the given value."] # [doc = ""] # [doc = " Usually one can avoid using this. It is useful when the application"] # [doc = " intentionally wants to leak the PRK secret, e.g. to implement"] # [doc = " `SSLKEYLOGFILE` functionality."] pub fn new_less_safe (algorithm : Algorithm , value : & [u8]) -> Self { Self (hmac :: Key :: new (algorithm . hmac_algorithm () , value)) } # [doc = " The [HKDF-Expand] operation."] # [doc = ""] # [doc = " [HKDF-Expand]: https://tools.ietf.org/html/rfc5869#section-2.3"] # [doc = ""] # [doc = " Fails if (and only if) `len` is too large."] # [inline] pub fn expand < 'a , L : KeyType > (& 'a self , info : & 'a [& 'a [u8]] , len : L ,) -> Result < Okm < 'a , L > , error :: Unspecified > { let len_cached = len . len () ; if len_cached > 255 * self . 0 . algorithm () . digest_algorithm () . output_len () { return Err (error :: Unspecified) ; } Ok (Okm { prk : self , info , len , len_cached , }) } }
};
}
