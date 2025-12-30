// Generated macro for impl_89 (impl)
macro_rules! Depcrate_string_cowimpl_89 {
() => {
// Module: crate::string_cow
// Provides: {"impl_89"}
// Dependencies: {}
impl < B : crate :: backend :: HeapStr > std :: ops :: Deref for KStringCowBase < '_ , B > { type Target = str ; # [inline] fn deref (& self) -> & str { self . as_str () } }
};
}
