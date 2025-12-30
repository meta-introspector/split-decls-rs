// Generated macro for impl_107 (impl)
macro_rules! Depcrate_keyimpl_107 {
() => {
// Module: crate::key
// Provides: {"impl_107"}
// Dependencies: {}
impl Key { # [doc = " Create a Key for the given type and path"] pub const fn for_path < T > (path : & str) -> Self where T : Schema + ? Sized , { Key (hash :: fnv1a64 :: hash_ty_path :: < T > (path)) } # [doc = " Create a key from a given 8-byte value"] # [doc = ""] # [doc = " NOTE: Since [`Key`]s should never be used to replace full type safety,"] # [doc = " creating a \"wrong\" Key should not be unsafe. However, using a \"wrong\""] # [doc = " key (which doesn't match the type being deserialized) may cause confusion,"] # [doc = " so manually creating keys should be avoided whenever possible."] pub const fn from_bytes (bytes : [u8 ; 8]) -> Self { Self (bytes) } # [doc = " Extract the bytes making up this key"] pub const fn to_bytes (& self) -> [u8 ; 8] { self . 0 } # [doc = " Compare 2 keys in const context."] pub const fn const_cmp (& self , other : & Self) -> bool { let mut i = 0 ; while i < self . 0 . len () { if self . 0 [i] != other . 0 [i] { return false ; } i += 1 ; } true } }
};
}
