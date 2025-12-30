// Generated macro for impl_323 (impl)
macro_rules! Depcrate_hkdfimpl_323 {
() => {
// Module: crate::hkdf
// Provides: {"impl_323"}
// Dependencies: {}
impl Salt { # [doc = " Constructs a new `Salt` with the given value based on the given digest"] # [doc = " algorithm."] # [doc = ""] # [doc = " Constructing a `Salt` is relatively expensive so it is good to reuse a"] # [doc = " `Salt` object instead of re-constructing `Salt`s with the same value."] pub fn new (algorithm : Algorithm , value : & [u8]) -> Self { Self (hmac :: Key :: new (algorithm . 0 , value)) } # [doc = " The [HKDF-Extract] operation."] # [doc = ""] # [doc = " [HKDF-Extract]: https://tools.ietf.org/html/rfc5869#section-2.2"] pub fn extract (& self , secret : & [u8]) -> Prk { let salt = & self . 0 ; let prk = hmac :: sign (salt , secret) ; Prk (hmac :: Key :: new (salt . algorithm () , prk . as_ref ())) } # [doc = " The algorithm used to derive this salt."] # [inline] pub fn algorithm (& self) -> Algorithm { Algorithm (self . 0 . algorithm ()) } }
};
}
