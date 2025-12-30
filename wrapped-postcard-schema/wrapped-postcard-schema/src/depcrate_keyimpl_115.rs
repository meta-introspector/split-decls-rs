// Generated macro for impl_115 (impl)
macro_rules! Depcrate_keyimpl_115 {
() => {
// Module: crate::key
// Provides: {"impl_115"}
// Dependencies: {}
impl Key { # [doc = " Create a Key for the given type and path"] pub const fn for_path < T > (path : & str) -> Self where T : Schema + ? Sized , { Key (hash :: fnv1a64 :: hash_ty_path :: < T > (path)) } # [doc = " Unsafely create a key from a given 8-byte value"] # [doc = ""] # [doc = " ## Safety"] # [doc = ""] # [doc = " This MUST only be used with pre-calculated values. Incorrectly"] # [doc = " created keys could lead to the improper deserialization of"] # [doc = " messages."] pub const unsafe fn from_bytes (bytes : [u8 ; 8]) -> Self { Self (bytes) } # [doc = " Extract the bytes making up this key"] pub const fn to_bytes (& self) -> [u8 ; 8] { self . 0 } # [doc = " Compare 2 keys in const context."] pub const fn const_cmp (& self , other : & Self) -> bool { let mut i = 0 ; while i < self . 0 . len () { if self . 0 [i] != other . 0 [i] { return false ; } i += 1 ; } true } }
};
}
