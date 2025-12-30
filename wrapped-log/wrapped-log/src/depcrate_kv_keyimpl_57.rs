// Generated macro for impl_57 (impl)
macro_rules! Depcrate_kv_keyimpl_57 {
() => {
// Module: crate::kv::key
// Provides: {"impl_57"}
// Dependencies: {}
impl < 'k > Key < 'k > { # [doc = " Get a key from a borrowed string."] pub fn from_str (key : & 'k str) -> Self { Key { key } } # [doc = " Get a borrowed string from this key."] # [doc = ""] # [doc = " The lifetime of the returned string is bound to the borrow of `self` rather"] # [doc = " than to `'k`."] pub fn as_str (& self) -> & str { self . key } # [doc = " Try get a borrowed string for the lifetime `'k` from this key."] # [doc = ""] # [doc = " If the key is a borrow of a longer lived string, this method will return `Some`."] # [doc = " If the key is internally buffered, this method will return `None`."] pub fn to_borrowed_str (& self) -> Option < & 'k str > { Some (self . key) } }
};
}
