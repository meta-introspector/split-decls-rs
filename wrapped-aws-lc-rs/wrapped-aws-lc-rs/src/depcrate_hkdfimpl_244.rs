// Generated macro for impl_244 (impl)
macro_rules! Depcrate_hkdfimpl_244 {
() => {
// Module: crate::hkdf
// Provides: {"impl_244"}
// Dependencies: {}
impl Salt { # [doc = " Constructs a new `Salt` with the given value based on the given digest"] # [doc = " algorithm."] # [doc = ""] # [doc = " Constructing a `Salt` is relatively expensive so it is good to reuse a"] # [doc = " `Salt` object instead of re-constructing `Salt`s with the same value."] # [doc = ""] # [doc = " # Panics"] # [doc = " `new` panics if the salt length exceeds the limit"] # [must_use] pub fn new (algorithm : Algorithm , value : & [u8]) -> Self { Salt :: try_new (algorithm , value) . expect ("Salt length limit exceeded.") } fn try_new (algorithm : Algorithm , value : & [u8]) -> Result < Salt , Unspecified > { let salt_len = value . len () ; if salt_len > MAX_HKDF_SALT_LEN { return Err (Unspecified) ; } let mut salt_bytes = [0u8 ; MAX_HKDF_SALT_LEN] ; salt_bytes [0 .. salt_len] . copy_from_slice (value) ; Ok (Self { algorithm , bytes : salt_bytes , len : salt_len , }) } # [doc = " The [HKDF-Extract] operation."] # [doc = ""] # [doc = " [HKDF-Extract]: https://tools.ietf.org/html/rfc5869#section-2.2"] # [doc = ""] # [doc = " # Panics"] # [doc = " Panics if the extract operation is unable to be performed"] # [inline] # [must_use] pub fn extract (& self , secret : & [u8]) -> Prk { Prk { algorithm : self . algorithm , mode : PrkMode :: ExtractExpand { secret : Arc :: from (ZeroizeBoxSlice :: from (secret)) , salt : self . bytes , salt_len : self . len , } , } } # [doc = " The algorithm used to derive this salt."] # [inline] # [must_use] pub fn algorithm (& self) -> Algorithm { Algorithm (self . algorithm . hmac_algorithm ()) } }
};
}
