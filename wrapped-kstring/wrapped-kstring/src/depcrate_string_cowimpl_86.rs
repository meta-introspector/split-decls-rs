// Generated macro for impl_86 (impl)
macro_rules! Depcrate_string_cowimpl_86 {
() => {
// Module: crate::string_cow
// Provides: {"impl_86"}
// Dependencies: {}
impl < B > KStringCowBase < '_ , B > { # [doc = " Create a new empty `KStringCowBase`."] # [inline] # [must_use] pub const fn new () -> Self { Self :: from_static ("") } # [doc = " Create a reference to a `'static` data."] # [inline] # [must_use] pub const fn from_static (other : & 'static str) -> Self { Self { inner : KStringCowInner :: Owned (KStringBase :: from_static (other)) , } } }
};
}
