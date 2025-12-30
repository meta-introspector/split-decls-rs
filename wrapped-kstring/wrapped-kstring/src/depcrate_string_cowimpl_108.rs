// Generated macro for impl_108 (impl)
macro_rules! Depcrate_string_cowimpl_108 {
() => {
// Module: crate::string_cow
// Provides: {"impl_108"}
// Dependencies: {}
impl < 's , B : crate :: backend :: HeapStr > From < KStringRef < 's > > for KStringCowBase < 's , B > { # [inline] fn from (other : KStringRef < 's >) -> Self { match other . inner { KStringRefInner :: Borrowed (s) => Self :: from_ref (s) , KStringRefInner :: Singleton (s) => Self :: from_static (s) , } } }
};
}
