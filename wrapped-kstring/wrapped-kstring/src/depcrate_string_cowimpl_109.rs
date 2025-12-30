// Generated macro for impl_109 (impl)
macro_rules! Depcrate_string_cowimpl_109 {
() => {
// Module: crate::string_cow
// Provides: {"impl_109"}
// Dependencies: {}
impl < 's , B : crate :: backend :: HeapStr > From < & 's KStringRef < 's > > for KStringCowBase < 's , B > { # [inline] fn from (other : & 's KStringRef < 's >) -> Self { match other . inner { KStringRefInner :: Borrowed (s) => Self :: from_ref (s) , KStringRefInner :: Singleton (s) => Self :: from_static (s) , } } }
};
}
