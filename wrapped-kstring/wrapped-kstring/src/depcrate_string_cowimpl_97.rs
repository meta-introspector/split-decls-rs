// Generated macro for impl_97 (impl)
macro_rules! Depcrate_string_cowimpl_97 {
() => {
// Module: crate::string_cow
// Provides: {"impl_97"}
// Dependencies: {}
impl < B : crate :: backend :: HeapStr > std :: hash :: Hash for KStringCowBase < '_ , B > { # [inline] fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { self . as_str () . hash (state) ; } }
};
}
