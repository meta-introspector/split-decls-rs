// Generated macro for impl_128 (impl)
macro_rules! Depcrate_string_refimpl_128 {
() => {
// Module: crate::string_ref
// Provides: {"impl_128"}
// Dependencies: {}
impl KStringRefInner < '_ > { # [inline] # [allow (clippy :: wrong_self_convention)] fn to_owned < B : crate :: backend :: HeapStr > (& self) -> KStringBase < B > { match self { Self :: Borrowed (s) => KStringBase :: from_ref (s) , Self :: Singleton (s) => KStringBase :: from_static (s) , } } # [inline] fn as_str (& self) -> & str { match self { Self :: Borrowed (s) => s , Self :: Singleton (s) => s , } } # [inline] fn into_mut (self) -> StdString { self . as_str () . to_owned () } }
};
}
